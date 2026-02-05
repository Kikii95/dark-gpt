// Setup wizard commands

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use tokio::process::Command as TokioCommand;
use tokio::time::{timeout, Duration};

const CMD_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Serialize, Deserialize)]
pub struct Prerequisites {
    pub os: OsInfo,
    pub docker: DependencyStatus,
    pub ollama: DependencyStatus,
    pub model_dolphin: bool,
    pub installed_models: Vec<String>,
    pub https_configured: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsInfo {
    pub name: String,
    pub version: String,
    pub arch: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyStatus {
    pub installed: bool,
    pub running: bool,
    pub version: Option<String>,
    pub download_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetupState {
    pub completed: bool,
    pub current_step: u32,
    pub total_steps: u32,
    pub prerequisites_ok: bool,
    pub model_downloaded: bool,
    pub services_configured: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppSettings {
    pub default_model: String,
    pub auto_start_services: bool,
    pub check_updates: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AvailableModel {
    pub name: String,
    pub size: String,
    pub ram_required: String,
    pub recommended: bool,
}

/// Get list of available Dolphin models
#[tauri::command]
pub fn get_available_models() -> Vec<AvailableModel> {
    vec![
        AvailableModel {
            name: "dolphin-phi".into(),
            size: "1.6 GB".into(),
            ram_required: "4 GB".into(),
            recommended: false,
        },
        AvailableModel {
            name: "dolphin-llama3:8b".into(),
            size: "4.7 GB".into(),
            ram_required: "8 GB".into(),
            recommended: true,
        },
        AvailableModel {
            name: "dolphin-mixtral:8x7b".into(),
            size: "26 GB".into(),
            ram_required: "32 GB".into(),
            recommended: false,
        },
    ]
}

/// Detect system prerequisites
#[tauri::command]
pub async fn detect_prerequisites() -> Result<Prerequisites, String> {
    tracing::info!("Detecting system prerequisites");

    // OS Info
    let os = OsInfo {
        name: std::env::consts::OS.to_string(),
        version: get_os_version(),
        arch: std::env::consts::ARCH.to_string(),
    };

    // Run Docker and Ollama detection concurrently
    let (docker, ollama, installed_models) = tokio::join!(
        detect_docker(),
        detect_ollama(),
        get_installed_dolphin_models(),
    );

    let model_dolphin = installed_models
        .iter()
        .any(|m| m.starts_with("dolphin-llama3:8b"));

    // Check HTTPS configuration
    let https_configured = check_https_configured();

    Ok(Prerequisites {
        os,
        docker,
        ollama,
        model_dolphin,
        installed_models,
        https_configured,
    })
}

async fn detect_docker() -> DependencyStatus {
    // Use tokio::process::Command (non-blocking) with a timeout
    let output = timeout(
        CMD_TIMEOUT,
        TokioCommand::new("docker")
            .args(["version", "--format", "{{.Server.Version}}"])
            .output(),
    )
    .await;

    match output {
        Ok(Ok(o)) if o.status.success() => {
            let version = String::from_utf8_lossy(&o.stdout).trim().to_string();

            // Check if daemon is responsive (also with timeout)
            let running = timeout(
                CMD_TIMEOUT,
                TokioCommand::new("docker").args(["info"]).output(),
            )
            .await
            .map(|r| r.map(|o| o.status.success()).unwrap_or(false))
            .unwrap_or(false);

            DependencyStatus {
                installed: true,
                running,
                version: Some(version),
                download_url: None,
            }
        }
        _ => DependencyStatus {
            installed: false,
            running: false,
            version: None,
            download_url: Some(get_docker_download_url()),
        },
    }
}

async fn detect_ollama() -> DependencyStatus {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
        .unwrap();

    match client
        .get("http://localhost:11434/api/version")
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => {
            let version: serde_json::Value = response.json().await.unwrap_or_default();
            DependencyStatus {
                installed: true,
                running: true,
                version: version["version"].as_str().map(|s| s.to_string()),
                download_url: None,
            }
        }
        _ => {
            // Check if binary exists (with timeout)
            let installed = check_binary_exists("ollama").await;

            DependencyStatus {
                installed,
                running: false,
                version: None,
                download_url: Some(get_ollama_download_url()),
            }
        }
    }
}

/// Check if a binary exists on the system PATH (non-blocking + timeout)
async fn check_binary_exists(name: &str) -> bool {
    // Try `where` on Windows, `which` on Unix
    let cmd = if cfg!(target_os = "windows") {
        "where"
    } else {
        "which"
    };

    timeout(
        Duration::from_secs(2),
        TokioCommand::new(cmd).arg(name).output(),
    )
    .await
    .map(|r| r.map(|o| o.status.success()).unwrap_or(false))
    .unwrap_or(false)
}

async fn get_installed_dolphin_models() -> Vec<String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
        .unwrap();

    if let Ok(response) = client
        .get("http://localhost:11434/api/tags")
        .send()
        .await
    {
        if let Ok(data) = response.json::<serde_json::Value>().await {
            if let Some(models) = data["models"].as_array() {
                return models
                    .iter()
                    .filter_map(|m| m["name"].as_str().map(|s| s.to_string()))
                    .filter(|name| name.starts_with("dolphin"))
                    .collect();
            }
        }
    }
    vec![]
}

fn check_https_configured() -> bool {
    // Check if /etc/hosts has dark-gpt.local
    if let Ok(content) = std::fs::read_to_string("/etc/hosts") {
        return content.contains("dark-gpt.local");
    }
    // Windows hosts file
    if let Ok(content) =
        std::fs::read_to_string("C:\\Windows\\System32\\drivers\\etc\\hosts")
    {
        return content.contains("dark-gpt.local");
    }
    false
}

fn get_os_version() -> String {
    #[cfg(target_os = "linux")]
    {
        std::fs::read_to_string("/etc/os-release")
            .ok()
            .and_then(|content| {
                content
                    .lines()
                    .find(|l| l.starts_with("PRETTY_NAME="))
                    .map(|l| l.replace("PRETTY_NAME=", "").replace('"', ""))
            })
            .unwrap_or_else(|| "Linux".to_string())
    }
    #[cfg(target_os = "windows")]
    {
        "Windows".to_string()
    }
    #[cfg(target_os = "macos")]
    {
        "macOS".to_string()
    }
}

fn get_docker_download_url() -> String {
    #[cfg(target_os = "windows")]
    {
        "https://desktop.docker.com/win/stable/Docker%20Desktop%20Installer.exe".to_string()
    }
    #[cfg(target_os = "linux")]
    {
        "https://docs.docker.com/engine/install/ubuntu/".to_string()
    }
    #[cfg(target_os = "macos")]
    {
        "https://desktop.docker.com/mac/stable/Docker.dmg".to_string()
    }
}

fn get_ollama_download_url() -> String {
    #[cfg(target_os = "windows")]
    {
        "https://ollama.com/download/OllamaSetup.exe".to_string()
    }
    #[cfg(target_os = "linux")]
    {
        "https://ollama.com/install.sh".to_string()
    }
    #[cfg(target_os = "macos")]
    {
        "https://ollama.com/download/Ollama-darwin.zip".to_string()
    }
}

/// Get current setup state
#[tauri::command]
pub async fn get_setup_state(app: AppHandle) -> Result<SetupState, String> {
    let config_dir = app
        .path()
        .app_config_dir()
        .map_err(|e| format!("Config dir error: {}", e))?;

    let setup_file = config_dir.join("setup_complete");
    let completed = setup_file.exists();

    let prereqs = detect_prerequisites().await?;

    Ok(SetupState {
        completed,
        current_step: if completed { 4 } else { 0 },
        total_steps: 4,
        prerequisites_ok: prereqs.docker.running && prereqs.ollama.running,
        model_downloaded: prereqs.model_dolphin,
        services_configured: prereqs.https_configured,
    })
}

/// Save app settings
#[tauri::command]
pub async fn save_settings(app: AppHandle, settings: AppSettings) -> Result<(), String> {
    let config_dir = app
        .path()
        .app_config_dir()
        .map_err(|e| format!("Config dir error: {}", e))?;

    std::fs::create_dir_all(&config_dir)
        .map_err(|e| format!("Failed to create config dir: {}", e))?;

    let settings_file = config_dir.join("settings.json");
    let json = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;

    std::fs::write(&settings_file, json)
        .map_err(|e| format!("Failed to write settings: {}", e))?;

    // Mark setup as complete
    let setup_file = config_dir.join("setup_complete");
    std::fs::write(&setup_file, "1")
        .map_err(|e| format!("Failed to mark setup complete: {}", e))?;

    tracing::info!("Settings saved to {:?}", settings_file);
    Ok(())
}

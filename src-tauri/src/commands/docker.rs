// Docker management commands

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use tokio::process::Command as TokioCommand;
use tokio::time::{timeout, Duration};

const CMD_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug, Serialize, Deserialize)]
pub struct DockerStatus {
    pub installed: bool,
    pub running: bool,
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceStatus {
    pub name: String,
    pub running: bool,
    pub health: Option<String>,
}

/// Check if Docker is installed and running
#[tauri::command]
pub async fn check_docker() -> Result<DockerStatus, String> {
    tracing::debug!("Checking Docker status");

    let version_output = timeout(
        CMD_TIMEOUT,
        TokioCommand::new("docker")
            .args(["version", "--format", "{{.Server.Version}}"])
            .output(),
    )
    .await;

    match version_output {
        Ok(Ok(output)) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            tracing::info!("Docker version: {}", version);

            let running = timeout(
                CMD_TIMEOUT,
                TokioCommand::new("docker").args(["info"]).output(),
            )
            .await
            .map(|r| r.map(|o| o.status.success()).unwrap_or(false))
            .unwrap_or(false);

            Ok(DockerStatus {
                installed: true,
                running,
                version: Some(version),
            })
        }
        Ok(Ok(_)) => Ok(DockerStatus {
            installed: true,
            running: false,
            version: None,
        }),
        _ => Ok(DockerStatus {
            installed: false,
            running: false,
            version: None,
        }),
    }
}

/// Start Docker Compose services
#[tauri::command]
pub async fn start_services(app: AppHandle) -> Result<Vec<ServiceStatus>, String> {
    tracing::info!("Starting Docker services");

    let docker_dir = app
        .path()
        .resource_dir()
        .map_err(|e| format!("Failed to get resource dir: {}", e))?
        .join("docker");

    let compose_file = docker_dir.join("docker-compose.yml");

    if !compose_file.exists() {
        return Err(format!(
            "docker-compose.yml not found at {:?}",
            compose_file
        ));
    }

    let output = timeout(
        Duration::from_secs(60),
        TokioCommand::new("docker")
            .args([
                "compose",
                "-f",
                compose_file.to_str().unwrap(),
                "up",
                "-d",
            ])
            .output(),
    )
    .await
    .map_err(|_| "Timed out starting services (60s)".to_string())?
    .map_err(|e| format!("Failed to start services: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Docker compose failed: {}", stderr));
    }

    tracing::info!("Docker services started");

    Ok(vec![
        ServiceStatus {
            name: "caddy".to_string(),
            running: true,
            health: Some("healthy".to_string()),
        },
        ServiceStatus {
            name: "webui".to_string(),
            running: true,
            health: Some("starting".to_string()),
        },
    ])
}

/// Stop Docker Compose services
#[tauri::command]
pub async fn stop_services(app: AppHandle) -> Result<(), String> {
    tracing::info!("Stopping Docker services");

    let docker_dir = app
        .path()
        .resource_dir()
        .map_err(|e| format!("Failed to get resource dir: {}", e))?
        .join("docker");

    let compose_file = docker_dir.join("docker-compose.yml");

    let output = timeout(
        Duration::from_secs(30),
        TokioCommand::new("docker")
            .args([
                "compose",
                "-f",
                compose_file.to_str().unwrap(),
                "down",
            ])
            .output(),
    )
    .await
    .map_err(|_| "Timed out stopping services (30s)".to_string())?
    .map_err(|e| format!("Failed to stop services: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Docker compose down failed: {}", stderr));
    }

    tracing::info!("Docker services stopped");
    Ok(())
}

/// Get logs from a specific service
#[tauri::command]
pub async fn get_service_logs(
    app: AppHandle,
    service: String,
    lines: Option<u32>,
) -> Result<String, String> {
    let docker_dir = app
        .path()
        .resource_dir()
        .map_err(|e| format!("Failed to get resource dir: {}", e))?
        .join("docker");

    let compose_file = docker_dir.join("docker-compose.yml");
    let lines_str = lines.unwrap_or(100).to_string();

    let output = timeout(
        Duration::from_secs(10),
        TokioCommand::new("docker")
            .args([
                "compose",
                "-f",
                compose_file.to_str().unwrap(),
                "logs",
                "--tail",
                &lines_str,
                &service,
            ])
            .output(),
    )
    .await
    .map_err(|_| "Timed out fetching logs".to_string())?
    .map_err(|e| format!("Failed to get logs: {}", e))?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

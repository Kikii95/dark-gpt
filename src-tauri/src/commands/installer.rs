// Auto-installer for dependencies (Docker Desktop, Ollama)
// Windows-only silent install with streaming progress events

use serde::Serialize;
use std::path::PathBuf;
use tauri::{Emitter, Window};
use tokio::io::AsyncWriteExt;

const OLLAMA_INSTALLER_URL: &str = "https://ollama.com/download/OllamaSetup.exe";
const DOCKER_INSTALLER_URL: &str =
    "https://desktop.docker.com/win/main/amd64/Docker%20Desktop%20Installer.exe";

#[derive(Debug, Clone, Serialize)]
pub struct InstallProgress {
    pub dependency: String,
    pub phase: String, // downloading | installing | verifying | done | error
    pub downloaded: u64,
    pub total: u64,
    pub percent: f32,
    pub message: String,
}

// -- Helpers ------------------------------------------------------------------

fn emit_progress(
    window: &Window,
    dependency: &str,
    phase: &str,
    downloaded: u64,
    total: u64,
    message: &str,
) {
    let percent = if total > 0 {
        (downloaded as f32 / total as f32) * 100.0
    } else {
        0.0
    };
    let _ = window.emit(
        "install-progress",
        InstallProgress {
            dependency: dependency.to_string(),
            phase: phase.to_string(),
            downloaded,
            total,
            percent,
            message: message.to_string(),
        },
    );
}

fn format_bytes(bytes: u64) -> String {
    if bytes == 0 {
        return "0 B".to_string();
    }
    let units = ["B", "KB", "MB", "GB"];
    let k = 1024_f64;
    let i = ((bytes as f64).ln() / k.ln()).floor() as usize;
    let i = i.min(units.len() - 1);
    format!("{:.1} {}", bytes as f64 / k.powi(i as i32), units[i])
}

/// Download a file with chunked streaming + progress events.
async fn download_file(
    window: &Window,
    dependency: &str,
    url: &str,
    dest: &PathBuf,
) -> Result<(), String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(600))
        .build()
        .map_err(|e| format!("HTTP client error: {}", e))?;

    let mut response = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("Download request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("HTTP {} — download failed", response.status()));
    }

    let total = response.content_length().unwrap_or(0);
    let mut downloaded: u64 = 0;

    let mut file = tokio::fs::File::create(dest)
        .await
        .map_err(|e| format!("Cannot create temp file: {}", e))?;

    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|e| format!("Download stream error: {}", e))?
    {
        file.write_all(&chunk)
            .await
            .map_err(|e| format!("Write error: {}", e))?;
        downloaded += chunk.len() as u64;
        emit_progress(
            window,
            dependency,
            "downloading",
            downloaded,
            total,
            &format!(
                "Downloading... {} / {}",
                format_bytes(downloaded),
                format_bytes(total)
            ),
        );
    }

    file.flush()
        .await
        .map_err(|e| format!("Flush error: {}", e))?;

    // Sanity check: file shouldn't be empty
    let meta = tokio::fs::metadata(dest)
        .await
        .map_err(|e| format!("Cannot stat downloaded file: {}", e))?;
    if meta.len() < 1024 {
        let _ = tokio::fs::remove_file(dest).await;
        return Err("Downloaded file is suspiciously small — aborting".to_string());
    }

    Ok(())
}

// -- Ollama -------------------------------------------------------------------

/// Download and install Ollama silently (Windows only).
#[tauri::command]
pub async fn install_ollama(window: Window) -> Result<(), String> {
    if cfg!(not(target_os = "windows")) {
        return Err(
            "Auto-install is only available on Windows. Please install Ollama manually."
                .to_string(),
        );
    }

    tracing::info!("Starting Ollama auto-install");

    let temp_dir = std::env::temp_dir();
    let installer_path = temp_dir.join("OllamaSetup.exe");

    // Phase 1: Download
    emit_progress(&window, "ollama", "downloading", 0, 0, "Starting download...");
    download_file(&window, "ollama", OLLAMA_INSTALLER_URL, &installer_path).await?;

    // Phase 2: Silent install (Inno Setup flags)
    emit_progress(
        &window,
        "ollama",
        "installing",
        0,
        0,
        "Installing Ollama (this may take a moment)...",
    );

    let output = tokio::process::Command::new(&installer_path)
        .args(["/VERYSILENT", "/NORESTART", "/SP-"])
        .output()
        .await
        .map_err(|e| format!("Failed to run installer: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let _ = tokio::fs::remove_file(&installer_path).await;
        return Err(format!("Ollama installer failed: {}", stderr));
    }

    // Phase 3: Verify — poll the API
    emit_progress(
        &window,
        "ollama",
        "verifying",
        0,
        0,
        "Waiting for Ollama service...",
    );

    let client = reqwest::Client::new();
    let mut verified = false;

    for attempt in 1..=40 {
        emit_progress(
            &window,
            "ollama",
            "verifying",
            attempt,
            40,
            &format!("Waiting for Ollama... ({}/40)", attempt),
        );
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        if let Ok(resp) = client
            .get("http://localhost:11434/api/version")
            .send()
            .await
        {
            if resp.status().is_success() {
                verified = true;
                break;
            }
        }
    }

    // Fallback: try to start ollama serve manually
    if !verified {
        emit_progress(
            &window,
            "ollama",
            "verifying",
            0,
            0,
            "Starting Ollama service manually...",
        );
        let _ = tokio::process::Command::new("ollama")
            .arg("serve")
            .spawn();

        for attempt in 1..=15 {
            emit_progress(
                &window,
                "ollama",
                "verifying",
                attempt,
                15,
                &format!("Retrying... ({}/15)", attempt),
            );
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;

            if let Ok(resp) = client
                .get("http://localhost:11434/api/version")
                .send()
                .await
            {
                if resp.status().is_success() {
                    verified = true;
                    break;
                }
            }
        }
    }

    // Cleanup
    let _ = tokio::fs::remove_file(&installer_path).await;

    if verified {
        emit_progress(
            &window,
            "ollama",
            "done",
            1,
            1,
            "Ollama installed successfully!",
        );
        tracing::info!("Ollama installed and verified");
        Ok(())
    } else {
        emit_progress(
            &window,
            "ollama",
            "error",
            0,
            0,
            "Ollama installed but service not responding",
        );
        Err(
            "Ollama installed but could not verify the service is running. Try restarting."
                .to_string(),
        )
    }
}

// -- Docker Desktop -----------------------------------------------------------

/// Download and install Docker Desktop silently (Windows only).
/// The UAC prompt will still appear — that's expected.
#[tauri::command]
pub async fn install_docker(window: Window) -> Result<(), String> {
    if cfg!(not(target_os = "windows")) {
        return Err(
            "Auto-install is only available on Windows. Please install Docker manually."
                .to_string(),
        );
    }

    tracing::info!("Starting Docker Desktop auto-install");

    let temp_dir = std::env::temp_dir();
    let installer_path = temp_dir.join("DockerDesktopInstaller.exe");

    // Phase 1: Download
    emit_progress(
        &window,
        "docker",
        "downloading",
        0,
        0,
        "Starting download...",
    );
    download_file(&window, "docker", DOCKER_INSTALLER_URL, &installer_path).await?;

    // Phase 2: Silent install (will show UAC prompt)
    emit_progress(
        &window,
        "docker",
        "installing",
        0,
        0,
        "Installing Docker Desktop (accept the UAC prompt)...",
    );

    let output = tokio::process::Command::new(&installer_path)
        .args(["install", "--quiet", "--accept-license"])
        .output()
        .await
        .map_err(|e| format!("Failed to run installer: {}", e))?;

    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let stderr_lower = stderr.to_lowercase();

    if !output.status.success() {
        let _ = tokio::fs::remove_file(&installer_path).await;

        // Docker often requires a restart after first install
        if stderr_lower.contains("restart") || stderr_lower.contains("reboot") {
            emit_progress(
                &window,
                "docker",
                "done",
                1,
                1,
                "Docker installed — system restart required",
            );
            return Err(
                "Docker Desktop installed but a system restart is required to complete setup."
                    .to_string(),
            );
        }

        return Err(format!("Docker installer failed: {}", stderr));
    }

    // Phase 3: Start Docker Desktop
    emit_progress(
        &window,
        "docker",
        "verifying",
        0,
        0,
        "Starting Docker Desktop...",
    );
    let docker_exe = r"C:\Program Files\Docker\Docker\Docker Desktop.exe";
    let _ = tokio::process::Command::new(docker_exe).spawn();

    // Phase 4: Wait for daemon — docker info
    let mut verified = false;
    for attempt in 1..=90 {
        emit_progress(
            &window,
            "docker",
            "verifying",
            attempt,
            90,
            &format!("Waiting for Docker daemon... ({}/90)", attempt),
        );
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        let check = tokio::process::Command::new("docker")
            .arg("info")
            .output()
            .await;

        if let Ok(out) = check {
            if out.status.success() {
                verified = true;
                break;
            }
        }
    }

    // Cleanup
    let _ = tokio::fs::remove_file(&installer_path).await;

    // Restart hint from stderr
    if stderr_lower.contains("restart") || stderr_lower.contains("reboot") {
        emit_progress(
            &window,
            "docker",
            "done",
            1,
            1,
            "Docker installed — restart may be needed for full functionality",
        );
        return Ok(());
    }

    if verified {
        emit_progress(
            &window,
            "docker",
            "done",
            1,
            1,
            "Docker Desktop installed and running!",
        );
        tracing::info!("Docker Desktop installed and verified");
    } else {
        // Don't hard-error — Docker is notoriously slow to cold-start
        emit_progress(
            &window,
            "docker",
            "done",
            1,
            1,
            "Docker installed — daemon may need more time to start",
        );
        tracing::warn!("Docker installed but daemon not yet responsive after 90s");
    }

    Ok(())
}

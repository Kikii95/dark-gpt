// Docker management commands

use serde::{Deserialize, Serialize};
use std::process::Command;
use tauri::{AppHandle, Manager};

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

    // Check if docker command exists
    let version_output = Command::new("docker")
        .args(["version", "--format", "{{.Server.Version}}"])
        .output();

    match version_output {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            tracing::info!("Docker version: {}", version);

            // Check if Docker daemon is running
            let info_output = Command::new("docker").args(["info"]).output();

            let running = info_output.map(|o| o.status.success()).unwrap_or(false);

            Ok(DockerStatus {
                installed: true,
                running,
                version: Some(version),
            })
        }
        Ok(_) => Ok(DockerStatus {
            installed: true,
            running: false,
            version: None,
        }),
        Err(_) => Ok(DockerStatus {
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

    // Get resource path for docker-compose.yml
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

    // Run docker compose up
    let output = Command::new("docker")
        .args([
            "compose",
            "-f",
            compose_file.to_str().unwrap(),
            "up",
            "-d",
        ])
        .output()
        .map_err(|e| format!("Failed to start services: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Docker compose failed: {}", stderr));
    }

    tracing::info!("Docker services started");

    // Return status of services
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

    let output = Command::new("docker")
        .args([
            "compose",
            "-f",
            compose_file.to_str().unwrap(),
            "down",
        ])
        .output()
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

    let output = Command::new("docker")
        .args([
            "compose",
            "-f",
            compose_file.to_str().unwrap(),
            "logs",
            "--tail",
            &lines_str,
            &service,
        ])
        .output()
        .map_err(|e| format!("Failed to get logs: {}", e))?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

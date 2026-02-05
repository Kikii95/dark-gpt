// Health check commands

use serde::{Deserialize, Serialize};
use tokio::process::Command as TokioCommand;
use tokio::time::{timeout, Duration};

const CMD_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthReport {
    pub docker: ServiceHealth,
    pub ollama: ServiceHealth,
    pub webui: ServiceHealth,
    pub caddy: ServiceHealth,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceHealth {
    pub name: String,
    pub status: HealthStatus,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Unhealthy,
    Unknown,
}

/// Check health of all services
#[tauri::command]
pub async fn check_all_services() -> Result<HealthReport, String> {
    tracing::debug!("Checking all services health");

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .map_err(|e| format!("Failed to create client: {}", e))?;

    // Run all health checks concurrently
    let (docker_health, ollama_health, webui_health, caddy_health) = tokio::join!(
        check_docker_health(),
        check_ollama_health(&client),
        check_webui_health(&client),
        check_caddy_health(&client),
    );

    Ok(HealthReport {
        docker: docker_health,
        ollama: ollama_health,
        webui: webui_health,
        caddy: caddy_health,
    })
}

async fn check_docker_health() -> ServiceHealth {
    let output = timeout(
        CMD_TIMEOUT,
        TokioCommand::new("docker").args(["info"]).output(),
    )
    .await;

    match output {
        Ok(Ok(o)) if o.status.success() => ServiceHealth {
            name: "Docker".to_string(),
            status: HealthStatus::Healthy,
            message: Some("Docker daemon running".to_string()),
        },
        Ok(Ok(_)) => ServiceHealth {
            name: "Docker".to_string(),
            status: HealthStatus::Unhealthy,
            message: Some("Docker daemon not responding".to_string()),
        },
        Ok(Err(_)) => ServiceHealth {
            name: "Docker".to_string(),
            status: HealthStatus::Unknown,
            message: Some("Docker not installed".to_string()),
        },
        Err(_) => ServiceHealth {
            name: "Docker".to_string(),
            status: HealthStatus::Unhealthy,
            message: Some("Docker check timed out (5s)".to_string()),
        },
    }
}

async fn check_ollama_health(client: &reqwest::Client) -> ServiceHealth {
    match client.get("http://localhost:11434/api/tags").send().await {
        Ok(response) if response.status().is_success() => ServiceHealth {
            name: "Ollama".to_string(),
            status: HealthStatus::Healthy,
            message: Some("Ollama running".to_string()),
        },
        Ok(_) => ServiceHealth {
            name: "Ollama".to_string(),
            status: HealthStatus::Unhealthy,
            message: Some("Ollama not responding correctly".to_string()),
        },
        Err(_) => ServiceHealth {
            name: "Ollama".to_string(),
            status: HealthStatus::Unhealthy,
            message: Some("Cannot connect to Ollama".to_string()),
        },
    }
}

async fn check_webui_health(client: &reqwest::Client) -> ServiceHealth {
    let urls = [
        "https://dark-gpt.local/health",
        "http://localhost:3002/health",
    ];

    for url in urls {
        if let Ok(response) = client.get(url).send().await {
            if response.status().is_success() {
                return ServiceHealth {
                    name: "Open-WebUI".to_string(),
                    status: HealthStatus::Healthy,
                    message: Some(format!("Accessible at {}", url)),
                };
            }
        }
    }

    ServiceHealth {
        name: "Open-WebUI".to_string(),
        status: HealthStatus::Unhealthy,
        message: Some("WebUI not accessible".to_string()),
    }
}

async fn check_caddy_health(client: &reqwest::Client) -> ServiceHealth {
    match client.get("https://dark-gpt.local/health").send().await {
        Ok(response) if response.status().is_success() => ServiceHealth {
            name: "Caddy".to_string(),
            status: HealthStatus::Healthy,
            message: Some("HTTPS proxy working".to_string()),
        },
        Ok(_) => ServiceHealth {
            name: "Caddy".to_string(),
            status: HealthStatus::Unhealthy,
            message: Some("Caddy responding but not healthy".to_string()),
        },
        Err(_) => ServiceHealth {
            name: "Caddy".to_string(),
            status: HealthStatus::Unhealthy,
            message: Some("Cannot connect to Caddy".to_string()),
        },
    }
}

/// Get the WebUI URL
#[tauri::command]
pub async fn get_webui_url() -> Result<String, String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(2))
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| format!("Client error: {}", e))?;

    if client
        .get("https://dark-gpt.local")
        .send()
        .await
        .is_ok()
    {
        return Ok("https://dark-gpt.local".to_string());
    }

    Ok("http://localhost:3002".to_string())
}

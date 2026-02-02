// Health check commands

use serde::{Deserialize, Serialize};

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
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| format!("Failed to create client: {}", e))?;

    // Check Docker
    let docker_health = check_docker_health().await;

    // Check Ollama
    let ollama_health = check_ollama_health(&client).await;

    // Check WebUI (via HTTPS)
    let webui_health = check_webui_health(&client).await;

    // Check Caddy
    let caddy_health = check_caddy_health(&client).await;

    Ok(HealthReport {
        docker: docker_health,
        ollama: ollama_health,
        webui: webui_health,
        caddy: caddy_health,
    })
}

async fn check_docker_health() -> ServiceHealth {
    let output = std::process::Command::new("docker")
        .args(["info"])
        .output();

    match output {
        Ok(o) if o.status.success() => ServiceHealth {
            name: "Docker".to_string(),
            status: HealthStatus::Healthy,
            message: Some("Docker daemon running".to_string()),
        },
        Ok(_) => ServiceHealth {
            name: "Docker".to_string(),
            status: HealthStatus::Unhealthy,
            message: Some("Docker daemon not responding".to_string()),
        },
        Err(_) => ServiceHealth {
            name: "Docker".to_string(),
            status: HealthStatus::Unknown,
            message: Some("Docker not installed".to_string()),
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
    // Try HTTPS first, then HTTP
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
    // Prefer HTTPS if available
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(2))
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

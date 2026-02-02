// Ollama API commands

use serde::{Deserialize, Serialize};
use tauri::{Emitter, Window};

const OLLAMA_BASE_URL: &str = "http://localhost:11434";

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaStatus {
    pub installed: bool,
    pub running: bool,
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    pub name: String,
    pub size: u64,
    pub modified_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub size: u64,
    pub parameter_size: Option<String>,
    pub quantization: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaVersion {
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaModels {
    models: Vec<OllamaModel>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaModel {
    name: String,
    size: u64,
    modified_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DownloadProgress {
    pub status: String,
    pub completed: u64,
    pub total: u64,
    pub percent: f32,
}

/// Check if Ollama is running
#[tauri::command]
pub async fn check_ollama() -> Result<OllamaStatus, String> {
    tracing::debug!("Checking Ollama status");

    let client = reqwest::Client::new();
    let url = format!("{}/api/version", OLLAMA_BASE_URL);

    match client.get(&url).send().await {
        Ok(response) if response.status().is_success() => {
            let version: OllamaVersion = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse version: {}", e))?;

            tracing::info!("Ollama version: {}", version.version);

            Ok(OllamaStatus {
                installed: true,
                running: true,
                version: Some(version.version),
            })
        }
        Ok(_) => Ok(OllamaStatus {
            installed: true,
            running: false,
            version: None,
        }),
        Err(_) => {
            // Check if ollama binary exists
            let exists = std::process::Command::new("which")
                .arg("ollama")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false);

            Ok(OllamaStatus {
                installed: exists,
                running: false,
                version: None,
            })
        }
    }
}

/// List installed models
#[tauri::command]
pub async fn list_models() -> Result<Vec<Model>, String> {
    tracing::debug!("Listing Ollama models");

    let client = reqwest::Client::new();
    let url = format!("{}/api/tags", OLLAMA_BASE_URL);

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to connect to Ollama: {}", e))?;

    if !response.status().is_success() {
        return Err("Ollama not responding".to_string());
    }

    let models: OllamaModels = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse models: {}", e))?;

    Ok(models
        .models
        .into_iter()
        .map(|m| Model {
            name: m.name,
            size: m.size,
            modified_at: m.modified_at,
        })
        .collect())
}

/// Pull a model (with progress events)
#[tauri::command]
pub async fn pull_model(window: Window, model_name: String) -> Result<(), String> {
    tracing::info!("Pulling model: {}", model_name);

    let client = reqwest::Client::new();
    let url = format!("{}/api/pull", OLLAMA_BASE_URL);

    let mut response = client
        .post(&url)
        .json(&serde_json::json!({ "name": model_name, "stream": true }))
        .send()
        .await
        .map_err(|e| format!("Failed to start pull: {}", e))?;

    // Stream progress events
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|e| format!("Stream error: {}", e))?
    {
        let text = String::from_utf8_lossy(&chunk);
        for line in text.lines() {
            if let Ok(progress) = serde_json::from_str::<serde_json::Value>(line) {
                let status = progress["status"].as_str().unwrap_or("").to_string();
                let completed = progress["completed"].as_u64().unwrap_or(0);
                let total = progress["total"].as_u64().unwrap_or(1);
                let percent = if total > 0 {
                    (completed as f32 / total as f32) * 100.0
                } else {
                    0.0
                };

                // Emit progress event to frontend
                let _ = window.emit(
                    "model-download-progress",
                    DownloadProgress {
                        status,
                        completed,
                        total,
                        percent,
                    },
                );
            }
        }
    }

    tracing::info!("Model {} pulled successfully", model_name);
    Ok(())
}

/// Get detailed model info
#[tauri::command]
pub async fn get_model_info(model_name: String) -> Result<ModelInfo, String> {
    let client = reqwest::Client::new();
    let url = format!("{}/api/show", OLLAMA_BASE_URL);

    let response = client
        .post(&url)
        .json(&serde_json::json!({ "name": model_name }))
        .send()
        .await
        .map_err(|e| format!("Failed to get model info: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Model {} not found", model_name));
    }

    let info: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse info: {}", e))?;

    Ok(ModelInfo {
        name: model_name,
        size: info["size"].as_u64().unwrap_or(0),
        parameter_size: info["details"]["parameter_size"]
            .as_str()
            .map(|s| s.to_string()),
        quantization: info["details"]["quantization_level"]
            .as_str()
            .map(|s| s.to_string()),
    })
}

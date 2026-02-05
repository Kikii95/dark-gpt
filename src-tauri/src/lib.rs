// Dark-GPT Library
// Core functionality for the Tauri application

mod commands;
mod services;
mod utils;

use tauri::Manager;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub fn run() {
    // Initialize logging
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive("dark_gpt=debug".parse().unwrap()))
        .init();

    tracing::info!("Starting Dark-GPT Desktop v0.6.2");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::docker::check_docker,
            commands::docker::start_services,
            commands::docker::stop_services,
            commands::docker::get_service_logs,
            commands::ollama::check_ollama,
            commands::ollama::list_models,
            commands::ollama::pull_model,
            commands::ollama::get_model_info,
            commands::health::check_all_services,
            commands::health::get_webui_url,
            commands::setup::detect_prerequisites,
            commands::setup::get_setup_state,
            commands::setup::save_settings,
            commands::setup::get_available_models,
        ])
        .setup(|app| {
            // Log app data directory
            if let Ok(data_dir) = app.path().app_data_dir() {
                tracing::info!("App data directory: {:?}", data_dir);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .unwrap_or_else(|e| {
            panic!(
                "Failed to start Dark-GPT: {}\n\n\
                 This is usually caused by a missing WebView2 runtime.\n\
                 Install it from: https://developer.microsoft.com/en-us/microsoft-edge/webview2/",
                e
            );
        });
}

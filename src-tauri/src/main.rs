// Dark-GPT Desktop Application
// Entry point for the Tauri application

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    dark_gpt_lib::run();
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{App, Window};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn main_window_ready(window: Window) {
    window.maximize().unwrap();
    window.show().unwrap();
}

fn main() {
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, main_window_ready])
        .build(tauri::generate_context!())
        .expect("error while building application");

    start_app(&app);

    app.run(|_, _| {});
}

fn start_app(app: &App) -> Window {
    let window = tauri::WindowBuilder::new(app, "main", tauri::WindowUrl::App("/".into()))
        .maximized(false)
        .visible(false)
        .build()
        .expect("error while creating window 'new'");

    window
}

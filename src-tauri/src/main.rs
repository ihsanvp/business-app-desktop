// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{App, Window};

#[tauri::command]
async fn app_window_ready(window: Window) {
    window.show().unwrap();
}

fn main() {
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![app_window_ready])
        .build(tauri::generate_context!())
        .expect("error while building application");

    let activated = false;

    if activated {
        create_main_window(&app);
    } else {
        create_activation_window(&app);
    }

    app.run(|_, _| {});
}

fn create_main_window(app: &App) -> Window {
    tauri::WindowBuilder::new(
        app,
        "main",
        tauri::WindowUrl::App("/src/windows/main/".into()),
    )
    .maximized(false)
    .visible(false)
    .build()
    .expect("error while creating window 'main'")
}

fn create_activation_window(app: &App) -> Window {
    tauri::WindowBuilder::new(
        app,
        "activation",
        tauri::WindowUrl::App("/src/windows/activation/".into()),
    )
    .maximized(false)
    .visible(false)
    .decorations(false)
    .center()
    .inner_size(800.0, 600.0)
    .build()
    .expect("error while creating window 'activation'")
}

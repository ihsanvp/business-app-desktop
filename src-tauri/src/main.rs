// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{App, AppHandle, Window};

#[tauri::command]
async fn window_ready(window: Window) {
    window.show().unwrap();
}

#[tauri::command]
async fn activation_complete(window: Window, handle: AppHandle) {
    let main_window = tauri::WindowBuilder::new(
        &handle,
        "main",
        tauri::WindowUrl::App("/src/windows/main/".into()),
    )
    .maximized(false)
    .visible(false)
    .build()
    .expect("error while creating window 'main'");

    main_window.set_title("Business App").unwrap();
    window.close().unwrap();
}

fn main() {
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![window_ready, activation_complete])
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

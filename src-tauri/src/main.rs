// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod activation;

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

#[tauri::command]
async fn save_activation(key: String, handle: AppHandle) {
    let hash = activation::get_device_hash(&key);
    let hash_file_path = activation::get_hash_save_path(&handle);
    let key_file_path = activation::get_key_save_path(&handle);

    activation::save_to_file(hash_file_path, hash);
    activation::save_to_file(key_file_path, key);
}

fn main() {
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            window_ready,
            activation_complete,
            save_activation
        ])
        .build(tauri::generate_context!())
        .expect("error while building application");

    let activated = activation::check_activation(&app.handle());

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

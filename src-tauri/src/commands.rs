use crate::activation;
use tauri::{AppHandle, Window};

#[tauri::command]
pub async fn window_ready(window: Window) {
    window.show().unwrap();
}

#[tauri::command]
pub async fn activation_complete(window: Window, handle: AppHandle) {
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
pub async fn get_device_fingerprint() -> String {
    activation::get_device_fingerprint()
}

#[tauri::command]
pub async fn save_activation(key: String, handle: AppHandle) -> String {
    let hash = activation::get_device_hash(&key);
    let hash_file_path = activation::get_hash_save_path(&handle);
    let key_file_path = activation::get_key_save_path(&handle);

    activation::save_to_file(hash_file_path, &hash);
    activation::save_to_file(key_file_path, &key);

    hash
}

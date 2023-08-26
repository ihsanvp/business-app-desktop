use crate::{activation, constants};
use tauri::{AppHandle, Window};

#[tauri::command]
pub async fn window_ready(window: Window) {
    window.show().unwrap();
}

#[tauri::command]
pub async fn activate_key(key: String, handle: AppHandle, window: Window) -> Result<(), String> {
    activation::validate_key_with_server(&key).await?;
    activation::activate_current_device(&key, &handle);

    let main_window = tauri::WindowBuilder::new(
        &handle,
        "main",
        tauri::WindowUrl::App("/src/windows/main/".into()),
    )
    .maximized(false)
    .visible(false)
    .build()
    .expect("error while creating window 'main'");

    main_window.set_title(constants::MAIN_WINDOW_TITLE).unwrap();
    main_window.maximize().unwrap();
    window.close().unwrap();

    Ok(())
}

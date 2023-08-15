// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod activation;
mod commands;
mod constants;

fn main() {
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::window_ready,
            commands::activation_complete,
            commands::save_activation,
            commands::get_device_fingerprint
        ])
        .build(tauri::generate_context!())
        .expect("error while building application");

    let activated = activation::check_activation(&app.handle());

    if activated {
        let main_window = tauri::WindowBuilder::new(
            &app,
            "main",
            tauri::WindowUrl::App("/src/windows/main/".into()),
        )
        .maximized(false)
        .visible(false)
        .build()
        .expect("error while creating window 'main'");
        main_window.set_title(constants::MAIN_WINDOW_TITLE).unwrap();
    } else {
        let activation_window = tauri::WindowBuilder::new(
            &app,
            "activation",
            tauri::WindowUrl::App("/src/windows/activation/".into()),
        )
        .maximized(false)
        .visible(false)
        .decorations(false)
        .center()
        .inner_size(800.0, 600.0)
        .build()
        .expect("error while creating window 'activation'");
        activation_window
            .set_title(constants::ACTIVATION_WINDOW_TITLE)
            .unwrap();
    }

    app.run(|_, _| {});
}

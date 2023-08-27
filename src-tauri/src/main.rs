// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayMenu, SystemTrayMenuItem};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

const MAIN_WINDOW_ID: &str = "main";

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let settings = CustomMenuItem::new("settings".to_string(), "Settings");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(settings);
    let system_tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            tauri::SystemTrayEvent::MenuItemClick { tray_id: _, id, .. } => match id.as_str() {
                "quit" => {
                    app.exit(0);
                }
                "hide" => {
                    let handle = app.app_handle();
                    let Some(window) = handle.get_window(MAIN_WINDOW_ID) else { return };
                    window.hide().unwrap();
                }
                "settings" => {
                    let handle = app.app_handle();
                    if handle.get_window(MAIN_WINDOW_ID).is_some() {
                        return;
                    }
                    let id = MAIN_WINDOW_ID.to_string();
                    let _window = tauri::WindowBuilder::new(
                        app,
                        id.clone(),
                        tauri::WindowUrl::App("/".into()),
                    )
                    .inner_size(400.0, 600.0)
                    .resizable(false)
                    .title(MAIN_WINDOW_ID)
                    .build()
                    .unwrap();
                }
                _ => {}
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![greet])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                // Make sure to prevent the exit until the quit menu item is clicked
                api.prevent_exit();
            }
            _ => {}
        });
}

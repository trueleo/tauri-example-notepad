#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{fs, path::PathBuf};

use tauri::{CustomMenuItem, Manager, Menu, MenuItem, Submenu};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn save(path: PathBuf, buffer: &str) -> Result<(), String> {
    fs::write(&path, buffer).map_err(|_| "Could not write to file".to_string())
}

fn main() {
    let savemenu = CustomMenuItem::new("save", "Save");
    let menu = Menu::new()
        .add_native_item(MenuItem::Quit)
        .add_item(savemenu);
    let submenu = Submenu::new("File", menu);
    let menu = Menu::new().add_submenu(submenu);

    tauri::Builder::default()
        .menu(menu)
        .setup(|app| {
            let _window = app.get_window("main").unwrap();
            let window = _window.clone();
            _window.on_menu_event(move |event| match event.menu_item_id() {
                "quit" => {
                    window.close().unwrap();
                    std::process::exit(0);
                }
                "save" => {
                    window.emit("callback_save", "").unwrap();
                }
                _ => {}
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![save])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

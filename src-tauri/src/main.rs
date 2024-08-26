// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::Write;

use tauri::{ CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent };
use tauri::Manager;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn make_new_note(file_data: &str, path: &str, file_name: &str) -> Result<String, String> {
    // Define the file path
    let final_location = format!("{}{}", path, file_name);

    // Attempt to create the file
    let mut file = match File::create(final_location) {
        Ok(file) => file,
        Err(err) => {
            return Err(format!("Error creating file: {}", err));
        }
    };

    // Attempt to write the string to the file
    if let Err(err) = file.write_all(file_data.as_bytes()) {
        return Err(format!("Error writing to file: {}", err));
    }

    // Return a success message
    Ok(
        format!(
            "A new note was created: \nFolder: {} \nFile: {} \nNote: {}",
            path,
            file_name,
            file_data
        )
    )
}

fn main() {
    let tray_menu = SystemTrayMenu::new(); // insert the menu items here
    let quit = CustomMenuItem::new("quit".to_string(), "Quit Application");
    let show = CustomMenuItem::new("show".to_string(), "Toogle Visibility");

    tauri::Builder
        ::default()
        .on_window_event(|event| {
            match event.event() {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    event.window().hide().unwrap();
                    api.prevent_close();
                }
                _ => {}
            }
        })
        .system_tray(SystemTray::new().with_menu(tray_menu.add_item(show).add_item(quit)))
        .on_system_tray_event(|app, event| {
            match event {
                SystemTrayEvent::LeftClick { position: _, size: _, .. } => {
                    println!("system tray received a left click");
                }
                SystemTrayEvent::RightClick { position: _, size: _, .. } => {
                    println!("system tray received a right click");
                }
                SystemTrayEvent::DoubleClick { position: _, size: _, .. } => {
                    println!("system tray received a double click");
                }
                SystemTrayEvent::MenuItemClick { id, .. } => {
                    match id.as_str() {
                        "quit" => {
                            std::process::exit(0);
                        }
                        "show" => {
                            let window = app.get_window("main").unwrap();
                            match window.is_visible() {
                                Ok(visible) => {
                                    if visible {
                                        window.hide().unwrap();
                                    } else {
                                        window.show().unwrap();
                                    }
                                }
                                Err(e) => {
                                    // Handle the error appropriately
                                    eprintln!("Error checking window visibility: {:?}", e);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![make_new_note])

        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

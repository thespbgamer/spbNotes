// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem,  SystemTray, SystemTrayMenu, SystemTrayEvent};
use tauri::Manager;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


fn main() {
    let tray_menu = SystemTrayMenu::new(); // insert the menu items here
    let quit = CustomMenuItem::new("quit".to_string(), "Quit Application");
let show = CustomMenuItem::new("show".to_string(), "Show");

  tauri::Builder::default().on_window_event(|event| match event.event() {
    tauri::WindowEvent::CloseRequested { api, .. } => {
      event.window().hide().unwrap();
      api.prevent_close();
    }
    _ => {}
  })
    .system_tray(SystemTray::new().with_menu(tray_menu.add_item(quit).add_item(show)))
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::LeftClick {
        position: _,
        size: _,
        ..
      } => {
        println!("system tray received a left click");
      }
      SystemTrayEvent::RightClick {
        position: _,
        size: _,
        ..
      } => {
        println!("system tray received a right click");
      }
      SystemTrayEvent::DoubleClick {
        position: _,
        size: _,
        ..
      } => {
        println!("system tray received a double click");
      }
      SystemTrayEvent::MenuItemClick { id, .. } => {
        match id.as_str() {
          "quit" => {
            std::process::exit(0);
          }
          "show" => {
            let window = app.get_window("main").unwrap();
            window.show().unwrap();
          }
          _ => {}
        }
      }_ => {}
    })
        .invoke_handler(tauri::generate_handler![greet])

    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

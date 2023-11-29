// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, Manager};

fn main() {
    // here `"quit".to_string()` defines the menu item id,
    // and the second parameter is the menu item label.
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    let menu = Menu::new()
    .add_native_item(MenuItem::Copy)
    .add_item(CustomMenuItem::new("hide", "Hide"))
    .add_submenu(submenu);

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| {
      match event.menu_item_id() {
        "quit" => {
          std::process::exit(0);
        }
        "close" => {
          event.window().close().unwrap();
        }
        _ => {}
      }
    })
        .setup(|app| {
      let main_window = app.get_window("main").unwrap();
      let menu_handle = main_window.menu_handle();
      std::thread::spawn(move || {
        // you can also `set_selected`, `set_enabled` and `set_native_image` (macOS only).
        menu_handle.get_item("item_id").set_title("New title");
      });
      Ok(())
    })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, Manager, Invoke, Window, command, generate_handler};

#[derive(Clone, serde::Serialize)]

struct Payload {
  message: String,
}
fn main() {
    #[tauri::command]
    fn my_custom_command(message: String) -> String {
      "Response from Rust".to_string()
    }
    // here `"quit".to_string()` defines the menu item id,
    // and the second parameter is the menu item label.
    // file
    let new_file = CustomMenuItem::new("new_file".to_string(), "New file"); // new file 
    let open_file = CustomMenuItem::new("open_file".to_string(), "Open file"); // open file 
    let save = CustomMenuItem::new("save".to_string(), "Save"); // save 
    let save_as = CustomMenuItem::new("save_as".to_string(), "Save as"); // save as 
    let export = CustomMenuItem::new("export".to_string(), "Export"); // export
    let quit = CustomMenuItem::new("quit".to_string(), "Quit"); // exit
    // edit
    let undo = CustomMenuItem::new("undo".to_string(), "Undo"); // undo
    let redo = CustomMenuItem::new("redo".to_string(), "Redo"); // redo
    let cut = CustomMenuItem::new("cut".to_string(), "Cut"); // cut 
    let copy = CustomMenuItem::new("copy".to_string(), "Copy"); // copy
    let paste = CustomMenuItem::new("paste".to_string(), "Paste"); // paste
    let delete = CustomMenuItem::new("delete".to_string(), "Delete"); // delete 
    let select_all = CustomMenuItem::new("select_all".to_string(), "Select all"); // select all 
    let settings = CustomMenuItem::new("settings".to_string(), "Settings"); // settings 
    // image 
    let resize = CustomMenuItem::new("resize".to_string(), "Resize"); // resize 
    let rotate = CustomMenuItem::new("rotate".to_string(), "Rotate"); // rotate 
    let flip = CustomMenuItem::new("flip".to_string(), "Flip"); // flip
    // view 
    let zoom = CustomMenuItem::new("zoom".to_string(), "Zoom"); // zoom 
    let zoom_out = CustomMenuItem::new("zoom_out".to_string(), "Zoom out"); // zoom out 
    // layers
    let new_layer = CustomMenuItem::new("new_layer".to_string(), "New layer"); // new layer 
    let delete_layer = CustomMenuItem::new("deleteL_lyer".to_string(), "Delete layer"); // delete layer 
    let unite_layers = CustomMenuItem::new("unite_layers".to_string(), "Unite layers"); // unite layers 
    // help 
    let help = CustomMenuItem::new("help".to_string(), "Help"); // help 
    let about = CustomMenuItem::new("about".to_string(), "About"); // about 
    let file = Submenu::new("File", Menu::new().add_item(new_file).add_item(open_file).add_item(save).add_item(save_as).add_item(export).add_item(quit)); 
    let edit = Submenu::new("Edit", Menu::new().add_item(undo).add_item(redo).add_item(cut).add_item(copy).add_item(paste).add_item(delete).add_item(select_all).add_item(settings)); 
    let image = Submenu::new("Image", Menu::new().add_item(resize).add_item(rotate).add_item(flip));
    let view = Submenu::new("View", Menu::new().add_item(zoom).add_item(zoom_out));
    let layers = Submenu::new("Layers", Menu::new().add_item(new_layer).add_item(delete_layer).add_item(unite_layers));
    let help = Submenu::new("Help", Menu::new().add_item(help).add_item(about));
    let menu = Menu::new()
    .add_submenu(file)
    .add_submenu(edit)
    .add_submenu(image)
    .add_submenu(view)
    .add_submenu(layers)
    .add_submenu(help);

    tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![greet])
        .menu(menu)
        .on_menu_event(|event| {
      match event.menu_item_id() {
        "quit" => {
          std::process::exit(0);
        }
        "new_file" => {
          event.window().emit("my_custom_command", "my_custom_command")
        }
        _ => {}
      }
    })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

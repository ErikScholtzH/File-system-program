// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file_system;

use file_system::FileSystem;
use std::env;

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};


#[tauri::command]
fn show_drives() -> String {
  match FileSystem::new() {
    Ok(file_system) => {
        let disks = file_system.get_disks();
        serde_json::to_string(&disks).unwrap_or_else(|_| String::new())
    }
    Err(err) => {
        // Handle the error case
        println!("Error creating FileSystem: {:?}", err);
        String::new() // TODO: determine what to return here
    }
  }
}

fn main() {
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    let menu = Menu::new()
        .add_native_item(MenuItem::CloseWindow)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![show_drives])
        .menu(menu)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    }


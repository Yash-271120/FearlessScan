// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod storage;
mod filesystem;
mod error;

use std::sync::Mutex;

use storage::get_volumes;
use filesystem::{read_directory,open_file};
use tauri::Manager;


pub struct MyState {
    counter: u8,
}

impl MyState {
    fn new() -> MyState {
        MyState { counter: 0 }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .setup(|app| {
            app.manage(Mutex::new(MyState::new()));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_volumes,
            read_directory,
            open_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[cfg(test)]
mod tests {

   
}

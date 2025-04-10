// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod command;
mod storage;
mod filesystem;

use std::sync::Mutex;

use command::{increase_counter, show_counter, yash};
use storage::get_volumes;
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello World!, {}! You've been greeted from Rust!", name)
}

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
            greet,
            yash,
            increase_counter,
            show_counter,
            get_volumes
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[cfg(test)]
mod tests {
    use crate::command::yash;

    #[test]
    fn it_works() {
        let result = yash("Pash");
        assert_eq!(result, "Yash says salam to Pash")
    }
}

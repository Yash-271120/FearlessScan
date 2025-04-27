// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod error;
mod filesystem;
mod search;
mod storage;

use std::{collections::HashMap, sync::{mpsc::Sender, Arc, Mutex}};

use filesystem::{open_file, read_directory};
use search::search_directory_fast;
use serde::{Deserialize, Serialize};
use storage::get_volumes;
use tauri::Manager;

#[derive(Debug, Serialize, Deserialize)]
pub enum FileKind {
    File,
    Directory,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CachedPath {
    #[serde(rename = "p")]
    file_path: String,
    #[serde(rename = "t")]
    file_kind: FileKind,
}

pub type VolumeCache = HashMap<String, Vec<CachedPath>>;

pub struct MyState {
    directory_change_event_channel_sender: Option<Sender<String>>,
    storage_cache: HashMap<String, VolumeCache>
}

impl MyState {
    fn new() -> MyState {
        MyState {
            directory_change_event_channel_sender: None,
            storage_cache: HashMap::new()
        }
    }
}

pub type SafeMyState = Arc<Mutex<MyState>>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .setup(|app| {
            app.manage(Arc::new(Mutex::new(MyState::new())));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_volumes,
            read_directory,
            open_file,
            search_directory_fast
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[cfg(test)]
mod tests {}

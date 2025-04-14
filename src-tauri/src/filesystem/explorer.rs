use std::sync::Arc;
use std::{fs, thread};
use notify::RecursiveMode;
use serde::Serialize;
use tauri::{AppHandle, Emitter};

use crate::error::MyError;
use crate::storage::DirectoryPath;
use crate::filesystem::{MyFSEventHandler, MyFSWatcher};

#[tauri::command]
pub fn read_directory(app: AppHandle,path: String) -> Result<Vec<DirectoryPath>, MyError> {
    let read_dir_result = fs::read_dir(&path)?;

    let mut folder_structure: Vec<DirectoryPath> = Vec::new();

    for entry in read_dir_result {
        let entry = entry?;
        folder_structure.push(DirectoryPath::from(&entry));
    }

    let safe_app = Arc::new(app);
    let mut watcher = MyFSWatcher::new(path, MyFSEventHandler::new(), safe_app);

    thread::spawn(move||{
        loop {
            let _ = watcher.watch(RecursiveMode::NonRecursive);
        }
    });

    Ok(folder_structure)
}

#[derive(Serialize, Clone)]
struct EventStruct {
    name: String,
    path: String,
}

#[tauri::command]
pub fn open_file(app:AppHandle) -> Result<String, MyError> {
    // Return `null` on success

    let payload = EventStruct {
        name: "Yash".to_string(),
        path: "C:\\Users\\narut\\Desktop\\tauri-tut\\tauri-tut\\src-tauri\\src\\filesystem\\explorer.rs".to_string(),
    };
    app.emit("yash-event", payload);

    Ok("Yash".to_string())
}

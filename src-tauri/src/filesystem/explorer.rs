use notify::RecursiveMode;
use serde::Serialize;
use std::sync::Arc;
use std::{fs, thread};
use tauri::{AppHandle, Emitter, State};

use crate::error::MyError;
use crate::filesystem::{MyFSEventHandler, MyFSWatcher};
use crate::storage::DirectoryPath;
use crate::SafeMyState;

#[tauri::command]
pub fn read_directory(
    state_mux: State<'_, SafeMyState>,
    app: AppHandle,
    path: String,
) -> Result<Vec<DirectoryPath>, MyError> {
    let read_dir_result = fs::read_dir(&path)?;

    let mut folder_structure: Vec<DirectoryPath> = Vec::new();

    for entry in read_dir_result {
        let entry = entry?;
        folder_structure.push(DirectoryPath::from(&entry));
    }

    let safe_app = Arc::new(app);

    let mut gaurded_state = state_mux.lock().unwrap();

    if let Some(directory_event_sender) = &gaurded_state.directory_change_event_channel_sender {
        directory_event_sender.send(path).unwrap();
    } else {
        let (mut watcher, directory_event_sender) =
            MyFSWatcher::new(MyFSEventHandler::new(), safe_app);

        thread::spawn(move || {
            watcher.watch(RecursiveMode::NonRecursive).unwrap();
            println!("Thread exited!!")
        });

        (&directory_event_sender).send(path).unwrap();

        gaurded_state.directory_change_event_channel_sender = Some(directory_event_sender);
    }

    Ok(folder_structure)
}

#[derive(Serialize, Clone)]
struct EventStruct {
    name: String,
    path: String,
}

use notify::RecursiveMode;
use std::path::PathBuf;
use std::sync::Arc;
use std::{fs, thread};
use tauri::{AppHandle, State};

use crate::error::MyError;
use crate::filesystem::{MyFSEventHandler, MyFSWatcher};
use crate::storage::DirectoryPath;
use crate::SafeMyState;

#[tauri::command]
pub fn open_file(path: String) -> Result<(), MyError> {
    match open::that(path) {
        Ok(_) => Ok(()),
        Err(err) => {
            println!("Error: {:?}", err);
            return Err(MyError::FileOpen);
        }
    }
}

#[tauri::command]
pub fn read_directory(
    state_mux: State<'_, SafeMyState>,
    app: AppHandle,
    path: String,
    mount_point: String,
) -> Result<Vec<DirectoryPath>, MyError> {
    let read_dir_result = fs::read_dir(&path)?;

    let mut folder_structure: Vec<DirectoryPath> = Vec::new();

    for entry in read_dir_result {
        let entry = entry?;
        folder_structure.push(DirectoryPath::from(&entry));
    }

    let safe_app = Arc::new(app);

    let mut gaurded_state = state_mux.lock().unwrap();
    let mut mount_point_pathbuf = PathBuf::new();
    mount_point_pathbuf.push(mount_point);

    if let Some(directory_event_sender) = &gaurded_state.directory_change_event_channel_sender {
        directory_event_sender.send(path).unwrap();
    } else {
        let (mut watcher, directory_event_sender) = MyFSWatcher::new(
            MyFSEventHandler::new(Arc::clone(&state_mux),mount_point_pathbuf),
            safe_app,
            &state_mux,
        );

        thread::spawn(move || {
            watcher.watch(RecursiveMode::NonRecursive).unwrap();
            println!("Thread exited!!")
        });

        let event_sender = &directory_event_sender;
        event_sender.send(path).unwrap();

        gaurded_state.directory_change_event_channel_sender = Some(directory_event_sender);
    }

    Ok(folder_structure)
}

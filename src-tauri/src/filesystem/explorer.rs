use fuzzy_matcher::skim::SkimMatcherV2;
use notify::RecursiveMode;
use rayon::prelude::*;
use std::path::Path;
use std::sync::Arc;
use std::{fs, thread};
use tauri::{AppHandle, State};

use crate::error::MyError;
use crate::filesystem::{MyFSEventHandler, MyFSWatcher};
use crate::storage::{DirectoryPath, SearchResult};
use crate::SafeMyState;

fn execute_search(path: &Path, query: &str, matcher: &SkimMatcherV2) -> Vec<SearchResult> {
    if !path.is_dir() {
        return vec![];
    }

    let entries: Vec<_> = fs::read_dir(path).unwrap().filter_map(Result::ok).collect();
    let mut dir_files: Vec<SearchResult> = entries
        .iter()
        .filter(|entry| entry.path().is_file())
        .map(|entry| SearchResult::try_from(&entry, matcher, query))
        .filter_map(|opt| opt)
        .collect();

    let nested_files: Vec<Vec<SearchResult>> = entries
        .into_par_iter()
        .filter(|entry| entry.path().is_dir())
        .map(|entry| execute_search(&entry.path(), query, matcher))
        .collect();

    for mut nested_file in nested_files {
        dir_files.append(&mut nested_file);
    }

    dir_files
}

#[tauri::command]
pub fn search_directory(path: String, query: String) -> Result<Vec<SearchResult>, MyError> {
    let matcher = SkimMatcherV2::default().smart_case();
    let path = Path::new(&path);
    let mut dir_files = execute_search(path, &query, &matcher);

    //sort by score
    dir_files.sort_by(|a, b| b.score.cmp(&a.score));

    Ok(dir_files)
}

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

        let event_sender = &directory_event_sender;
        event_sender.send(path).unwrap();

        gaurded_state.directory_change_event_channel_sender = Some(directory_event_sender);
    }

    Ok(folder_structure)
}

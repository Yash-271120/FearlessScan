use crossbeam::channel::{bounded, Receiver, Sender};
use fuzzy_matcher::skim::SkimMatcherV2;
use notify::RecursiveMode;
use rayon::prelude::*;
use std::collections::BinaryHeap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::{fs, thread};
use tauri::{AppHandle, Emitter, Listener, State};

use crate::error::MyError;
use crate::filesystem::{MyFSEventHandler, MyFSWatcher};
use crate::search::{
    execute_search_low_memory, SearchResult,
};
use crate::storage::DirectoryPath;
use crate::SafeMyState;

const FS_SEARCH_EVENT: &str = "search-event";
const FS_UNLISTEN_SEARCH_EVENT: &str = "unlisten-search-event";

// fn execute_search(path: &Path, query: &str, matcher: &SkimMatcherV2) -> Vec<SearchResult> {
//     if !path.is_dir() {
//         return vec![];
//     }
//
//     let entries: Vec<_> = fs::read_dir(path).unwrap().filter_map(Result::ok).collect();
//     let mut dir_files: Vec<SearchResult> = entries
//         .iter()
//         .filter(|entry| entry.path().is_file())
//         .map(|entry| SearchResult::try_from(&entry, matcher, query))
//         .filter_map(|opt| opt)
//         .collect();
//
//     let nested_files: Vec<Vec<SearchResult>> = entries
//         .into_par_iter()
//         .filter(|entry| entry.path().is_dir())
//         .map(|entry| execute_search(&entry.path(), query, matcher))
//         .collect();
//
//     println!("Results: {:?},{:?}", dir_files.len(), nested_files.len());
//
//     for mut nested_file in nested_files {
//         dir_files.append(&mut nested_file);
//     }
//
//     dir_files
// }
//
// fn execute_search_listener(path: &Path, query: &str, matcher: &SkimMatcherV2, app: &AppHandle) {
//     let entries: Vec<_> = fs::read_dir(path).unwrap().filter_map(Result::ok).collect();
//     let _dir_files: Vec<_> = entries
//         .iter()
//         .filter(|entry| entry.path().is_file())
//         .map(|entry| SearchResult::try_from(&entry, matcher, query))
//         .filter_map(|opt| opt)
//         .map(|entry| {
//             println!("Emitting event: {:?}", entry);
//             app.emit(FS_SEARCH_EVENT, entry).unwrap()
//         })
//         .collect();
//
//     let _nested_files: Vec<_> = entries
//         .into_par_iter()
//         .filter(|entry| entry.path().is_dir())
//         .map(|entry| execute_search_listener(&entry.path(), query, matcher, app))
//         .collect();
// }

const MAX_SEARCH_RESULTS: usize = 1000;

#[tauri::command]
pub fn search_directory(
    app: AppHandle,
    path: String,
    query: String,
) -> Result<Vec<SearchResult>, MyError> {
    let start = std::time::Instant::now();
    let matcher = SkimMatcherV2::default().smart_case();
    let path = Path::new(&path);
    // let mut dir_files = execute_search(path, &query, &matcher);

    let mut search_tree: BinaryHeap<SearchResult> = BinaryHeap::new();

    let (sender, receiver): (Sender<Option<SearchResult>>, Receiver<Option<SearchResult>>) =
        bounded(1024);

    let collector_thread = thread::spawn(move || {
        loop {
            let result = receiver.recv();
            if result.is_err() {
                break;
            }
            if let Some(result) = result.unwrap() {
                search_tree.push(result);
                if search_tree.len() > MAX_SEARCH_RESULTS {
                    search_tree.pop();
                }
            }
        }

        search_tree
    });

    execute_search_low_memory(&path, &query, &matcher, &sender);

    // let result = execute_search_optimized(&path, &query, &matcher);
    //sort by score
    // dir_files.sort_by(|a, b| b.score.cmp(&a.score));

    // Ok(dir_files)
    //
    println!("Time taken: {:?}", start.elapsed());

    Ok(collector_thread.join().unwrap().into_sorted_vec())
}

#[tauri::command]
pub fn search_directory_listener(
    app: AppHandle,
    path: String,
    query: String,
) -> Result<(), MyError> {
    let matcher = SkimMatcherV2::default().smart_case();
    let app_safe = Arc::new(app);
    let app = Arc::clone(&app_safe);
    // let task = tauri::async_runtime::spawn(async move {
    //     let path = Path::new(&path);
    //     execute_search_listener(path, &query, &matcher, &app);
    // });
    //
    // Arc::clone(&app_safe).once(FS_UNLISTEN_SEARCH_EVENT, move |_| {
    //     println!("Unlistening search event");
    // });

    Ok(())
}

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
            MyFSWatcher::new(MyFSEventHandler::new(), safe_app, &state_mux);

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

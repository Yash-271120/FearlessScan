use std::{path::Path, sync::Arc};

use crate::filesystem::MyFSEventHandler;
use notify::{recommended_watcher, RecommendedWatcher, RecursiveMode, Watcher};
use tauri::AppHandle;

pub struct MyFSWatcher {
    path: String,
    watcher: RecommendedWatcher,
}

impl MyFSWatcher {
    pub fn new(path: String, handler: MyFSEventHandler, app: Arc<AppHandle>) -> Self {
        let watcher = recommended_watcher(move |result| match result {
            Ok(event) => handler.handle_event(event, Arc::clone(&app)),
            Err(_) => panic!("Error watching file system!"),
        })
        .unwrap();

        Self { path, watcher }
    }

    pub fn watch(&mut self, mode: RecursiveMode) -> notify::Result<()> {
        let path = Path::new(&self.path);
        self.watcher.watch(path, mode)
    }
}

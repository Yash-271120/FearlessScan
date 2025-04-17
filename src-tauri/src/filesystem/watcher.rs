use std::{
    path::Path,
    sync::{
        mpsc::{Receiver, Sender},
        Arc,
    },
};

use crate::filesystem::MyFSEventHandler;
use notify::{recommended_watcher, RecommendedWatcher, RecursiveMode, Watcher};
use tauri::AppHandle;

pub struct MyFSWatcher {
    path: Option<String>,
    watcher: RecommendedWatcher,
    directory_change_event_channel_reciever: Receiver<String>,
}

impl MyFSWatcher {
    pub fn new(handler: MyFSEventHandler, app: Arc<AppHandle>) -> (Self, Sender<String>) {
        let (directory_change_event_channel_sender, directory_change_event_channel_reciever) =
            std::sync::mpsc::channel::<String>();
        let watcher = recommended_watcher(move |result| match result {
            Ok(event) => handler.handle_event(event, Arc::clone(&app)),
            Err(_) => panic!("Error watching file system!"),
        })
        .unwrap();

        (
            Self {
                path: None,
                watcher,
                directory_change_event_channel_reciever,
            },
            directory_change_event_channel_sender,
        )
    }

    pub fn watch(&mut self, mode: RecursiveMode) -> notify::Result<()> {
        for event in &self.directory_change_event_channel_reciever {
            let old_path = &self.path;
            if let Some(old_path) = old_path {
                self.watcher.unwatch(Path::new(old_path))?;
            }
            self.path = Some(event.clone());
            self.watcher.watch(Path::new(&event), mode)?;
        }

        Ok(())
    }
}

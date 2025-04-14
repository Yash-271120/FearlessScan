use std::sync::Arc;

use notify::event::CreateKind;
use serde::Serialize;
use tauri::{AppHandle, Emitter};

use crate::storage::DirectoryPath;

const FS_EVENT_NAME: &'static str = "fs-event";

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
enum MyFSEventKind {
    Create,
    Remove,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MyFSEvent {
    directory_path: DirectoryPath,
    kind: MyFSEventKind,
}

pub struct MyFSEventHandler {}

impl MyFSEventHandler {
    pub fn new() -> Self {
        MyFSEventHandler {}
    }

    pub fn handle_event(&self, event: notify::Event, app: Arc<AppHandle>) {
        match event.kind {
            notify::EventKind::Create(create_kind) => {
                let path = event.paths[0].clone();
                let file_name = path.file_name().unwrap().to_string_lossy().to_string();

                let directory_path = match create_kind {
                    CreateKind::Folder => DirectoryPath::Directory {
                        name: file_name,
                        path: path.to_string_lossy().to_string(),
                    },
                    CreateKind::File => DirectoryPath::File {
                        name: file_name,
                        path: path.to_string_lossy().to_string(),
                    },
                    _ => panic!("Unknown create kind"),
                };
                app.emit(
                    FS_EVENT_NAME,
                    MyFSEvent {
                        directory_path,
                        kind: MyFSEventKind::Create,
                    },
                )
                .unwrap();
            }
            notify::EventKind::Modify(modify_kind) => {
                println!("Modified {:#?} -> Path: {:?}", modify_kind, event.paths[0])
            }
            notify::EventKind::Remove(remove_kind) => {
                println!("Removed {:#?} -> Path: {:?}", remove_kind, event.paths[0])
            }
            notify::EventKind::Any => {}
            notify::EventKind::Access(_access_kind) => {}
            notify::EventKind::Other => {}
        };
    }
}

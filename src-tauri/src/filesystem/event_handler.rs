use std::sync::Arc;

use notify::event::{ModifyKind, RenameMode};
use serde::Serialize;
use tauri::{AppHandle, Emitter};

use crate::storage::DirectoryPath;

const FS_EVENT_NAME: &str = "fs-event";

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
            notify::EventKind::Create(_create_kind) => {
                let path = event.paths[0].clone();
                let file_name = path.file_name().unwrap().to_string_lossy().to_string();
                let is_directory = path.is_dir();

                let directory_path = match is_directory {
                    true => DirectoryPath::Directory {
                        name: file_name,
                        path: path.to_string_lossy().to_string(),
                    },
                    false => DirectoryPath::File {
                        name: file_name,
                        path: path.to_string_lossy().to_string(),
                    },
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
                let path = event.paths[0].clone();
                let file_name = path.file_name().unwrap().to_string_lossy().to_string();
                let is_directory = path.is_dir();

                let directory_path = match is_directory {
                    true => DirectoryPath::Directory {
                        name: file_name,
                        path: path.to_string_lossy().to_string(),
                    },
                    false => DirectoryPath::File {
                        name: file_name,
                        path: path.to_string_lossy().to_string(),
                    },
                };
                if let ModifyKind::Name(rename_mode) = modify_kind {
                    match rename_mode {
                        RenameMode::From => {
                            app.emit(
                                FS_EVENT_NAME,
                                MyFSEvent {
                                    directory_path,
                                    kind: MyFSEventKind::Remove,
                                },
                            )
                            .unwrap();
                        }
                        RenameMode::To => {
                            app.emit(
                                FS_EVENT_NAME,
                                MyFSEvent {
                                    directory_path,
                                    kind: MyFSEventKind::Create,
                                },
                            )
                            .unwrap();
                        }
                        _ => {}
                    }
                }
            }
            notify::EventKind::Remove(_remove_kind) => {
                let path = event.paths[0].clone();
                let file_name = path.file_name().unwrap().to_string_lossy().to_string();
                let is_directory = path.is_dir();

                let directory_path = match is_directory {
                    true => DirectoryPath::Directory {
                        name: file_name,
                        path: path.to_string_lossy().to_string(),
                    },
                    false => DirectoryPath::File {
                        name: file_name,
                        path: path.to_string_lossy().to_string(),
                    },
                };
                app.emit(
                    FS_EVENT_NAME,
                    MyFSEvent {
                        directory_path,
                        kind: MyFSEventKind::Remove,
                    },
                )
                .unwrap();
            }
            notify::EventKind::Any => {}
            notify::EventKind::Access(_access_kind) => {}
            notify::EventKind::Other => {}
        };
    }
}

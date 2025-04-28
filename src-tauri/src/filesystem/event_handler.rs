use std::{path::{Path, PathBuf}, sync::Arc};

use notify::event::{ModifyKind, RenameMode};
use serde::Serialize;
use tauri::{AppHandle, Emitter};

use crate::{storage::DirectoryPath, CachedPath, FileKind, MyState, SafeMyState, VolumeCache};

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

pub struct MyFSEventHandler {
    state_mux: SafeMyState,
    mount_point: PathBuf,
}

impl MyFSEventHandler {
    pub fn new(state_mux: SafeMyState, mount_point: PathBuf) -> Self {
        MyFSEventHandler {
            state_mux,
            mount_point,
        }
    }

    fn get_cache_map<'a>(&self, state: &'a mut MyState)-> &'a mut VolumeCache{
        let mount_point = self.mount_point.to_string_lossy().to_string();

        state.storage_cache.get_mut(&mount_point).unwrap_or_else(||{
            panic!("Failed to get cache map for mount point: {}", mount_point);
        })
    }

    fn handle_create(&self, path:&Path, app: Arc<AppHandle>){
        let mut state = self.state_mux.lock().unwrap();
        let current_volume = self.get_cache_map(&mut state);

        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
        let file_path = path.to_string_lossy().to_string();
        let file_kind = if path.is_dir(){
            FileKind::Directory
        }else {
            FileKind::File
        };


        let directory_path = match file_kind {
            FileKind::Directory => DirectoryPath::Directory {
                name: file_name.clone(),
                path: path.to_string_lossy().to_string(),
            },
            FileKind::File => DirectoryPath::File {
                name: file_name.clone(),
                path: path.to_string_lossy().to_string(),
            },
        };

        current_volume.entry(file_name).or_insert_with(|| vec![CachedPath{
            file_kind,
            file_path
        }]);

        app.emit(
            FS_EVENT_NAME,
            MyFSEvent {
                directory_path,
                kind: MyFSEventKind::Create,
            },
        )
        .unwrap();
    }

    pub fn handle_event(&self, event: notify::Event, app: Arc<AppHandle>) {
        match event.kind {
            notify::EventKind::Create(_create_kind) => {
                let path = event.paths[0].clone();
                self.handle_create(&path, app);
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

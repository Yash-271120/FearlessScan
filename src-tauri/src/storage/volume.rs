use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::{self, DirEntry, File},
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{
    storage::{
        bytes_to_gb,
        cache::{load_storage_cache, CACHE_FILE_PATH},
    },
    CachedPath, FileKind, SafeMyState,
};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use rayon::prelude::*;
use serde::Serialize;
use sysinfo::{Disk, Disks, System};
use tauri::{AppHandle, Emitter, State};
use walkdir::WalkDir;

use super::{
    cache::{run_cache_poll, save_storage_cache},
    is_user_facing_volume,
};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    name: String,
    mount_point: PathBuf,
    available_gb: u16,
    used_gb: u16,
    total_gb: u16,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum DirectoryPath {
    File { name: String, path: String },
    Directory { name: String, path: String },
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndexingStartedEventPayload {
    total_items: usize,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndexingProgressEventPayload {
    current_item: usize,
    total_items: usize,
    percentage: f32,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndexingFinishedEventPayload {
    total_processed: usize,
    elapsed_time_ms: u64,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccumulatingStartedEventPayload {
    mount_point: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccumulatingFinishedEventPayload {
    mount_point: String,
    items_found: usize,
}

impl Volume {
    fn from(disk: &Disk) -> Self {
        let used_bytes = disk.total_space() - disk.available_space();
        let available_gb = bytes_to_gb(disk.available_space());
        let used_gb = bytes_to_gb(used_bytes);
        let total_gb = bytes_to_gb(disk.total_space());

        let name = {
            let volume_name = disk.name().to_str().unwrap();
            match volume_name.is_empty() {
                true => "Local Volume",
                false => volume_name,
            }
            .to_string()
        };

        let mount_point = disk.mount_point().to_path_buf();

        Self {
            name,
            available_gb,
            used_gb,
            total_gb,
            mount_point,
        }
    }

    fn create_cache(&self, state_mux: &SafeMyState, app_handle: &AppHandle, total_items: usize) {
        let start_time = std::time::Instant::now();
        let state = &mut state_mux.lock().unwrap();

        let volume_cache = state
            .storage_cache
            .entry(self.mount_point.to_string_lossy().to_string())
            .or_insert_with(HashMap::new);

        let storage_cache = Arc::new(Mutex::new(volume_cache));
        
        let directory_walker = WalkDir::new(self.mount_point.clone());
        
        // Counter for processed items
        let counter = Arc::new(Mutex::new(0usize));
        
        directory_walker
            .into_iter()
            .par_bridge()
            .filter_map(Result::ok)
            .for_each(|entry| {
                let file_type = entry.file_type();
                let file_type = if file_type.is_dir() {
                    FileKind::Directory
                } else {
                    FileKind::File
                };

                let entity_path = entry.path().to_string_lossy().to_string();
                let entity_name = entry.file_name().to_string_lossy().to_string();

                let mut guard = storage_cache.lock().unwrap();
                guard
                    .entry(entity_name)
                    .or_insert_with(Vec::default)
                    .push(CachedPath {
                        file_path: entity_path,
                        file_kind: file_type,
                    });
                
                // Update counter and emit progress event
                let mut count = counter.lock().unwrap();
                *count += 1;
                
                if *count % 100 == 0 || *count == 1 { // Emit every 100 items to avoid flooding
                    let percentage = (*count as f32 / total_items as f32) * 100.0;
                    let _ = app_handle.emit("indexing-progress", IndexingProgressEventPayload {
                        current_item: *count,
                        total_items,
                        percentage,
                    });
                }
            });
            
        // Emit indexing finished event
        let elapsed = start_time.elapsed();
        let total_processed = *counter.lock().unwrap();
        let _ = app_handle.emit("indexing-finished", IndexingFinishedEventPayload {
            total_processed,
            elapsed_time_ms: elapsed.as_millis() as u64,
        });
    }
}

impl DirectoryPath {
    pub fn from(dir_entry: &DirEntry) -> Self {
        let file_name = dir_entry.file_name().into_string().unwrap();
        let file_path = dir_entry.path().to_string_lossy().to_string();

        match dir_entry.path().is_dir() {
            true => DirectoryPath::Directory {
                name: file_name,
                path: file_path,
            },
            false => DirectoryPath::File {
                name: file_name,
                path: file_path,
            },
        }
    }
}

#[tauri::command]
pub async fn get_volumes(app_handle: AppHandle, state_mux: State<'_, SafeMyState>) -> Result<Vec<Volume>, String> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut cache_file_exists = fs::metadata(CACHE_FILE_PATH.as_str()).is_ok();

    if !cache_file_exists {
        File::create(CACHE_FILE_PATH.as_str()).unwrap();
    } else {
        cache_file_exists = load_storage_cache(&state_mux);
    }

    println!("Cache file exists: {}", cache_file_exists);

    let sys_disks = Disks::new_with_refreshed_list();

    // Calculate total items to index across all mount points
    let total_items_to_index = if !cache_file_exists {
        // Emit accumulating started event
        app_handle.emit("accumulating-started", AccumulatingStartedEventPayload {
            mount_point: "all".to_string(),
        }).unwrap();
        
        let start_time = std::time::Instant::now();
        
        let total = sys_disks
            .iter()
            .filter(|disk| {
                let name = disk.name().to_string_lossy().to_string();
                let mount_point = disk.mount_point().to_string_lossy().to_string();
                is_user_facing_volume(&name, &mount_point)
            })
            .map(|disk| {
                let mount_point = disk.mount_point().to_string_lossy().to_string();
                app_handle.emit("accumulating-started", AccumulatingStartedEventPayload {
                    mount_point: mount_point.clone(),
                }).unwrap();
                
                let count = WalkDir::new(disk.mount_point())
                    .into_iter()
                    .filter_map(Result::ok)
                    .count();
                    
                println!("Mount point {} has {} items", mount_point, count);
                
                app_handle.emit("accumulating-finished", AccumulatingFinishedEventPayload {
                    mount_point,
                    items_found: count,
                }).unwrap();
                
                count
            })
            .sum::<usize>();
        
        let elapsed = start_time.elapsed();
        
        // Emit accumulating finished event for all mount points
        app_handle.emit("accumulating-finished", AccumulatingFinishedEventPayload {
            mount_point: "all".to_string(),
            items_found: total,
        }).unwrap();
        
        total
    } else {
        0 // No indexing needed if cache exists
    };

    if !cache_file_exists && total_items_to_index > 0 {
        println!("Total items to index: {}", total_items_to_index);
        // Optional: Emit an event to the frontend with the total count
        app_handle.emit("indexing-started", IndexingStartedEventPayload { 
            total_items: total_items_to_index 
        }).unwrap();
    }

    let disks: Vec<Volume> = sys_disks
        .iter()
        .filter(|disk| {
            let name = disk.name().to_string_lossy().to_string();
            let mount_point = disk.mount_point().to_string_lossy().to_string();

            is_user_facing_volume(&name, &mount_point)
        })
        .map(|disk| {
            let volume = Volume::from(disk);

            if !cache_file_exists {
                volume.create_cache(&state_mux, &app_handle, total_items_to_index);
            }
            volume
        })
        .collect();

    save_storage_cache(&state_mux);
    run_cache_poll(&state_mux);

    Ok(disks)
}

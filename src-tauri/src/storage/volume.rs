use std::{fs::DirEntry, path::PathBuf};

use crate::storage::bytes_to_gb;
use serde::Serialize;
use sysinfo::{Disk, Disks, System};

use super::is_user_facing_volume;

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
pub fn get_volumes() -> Result<Vec<Volume>, String> {
    let mut sys = System::new_all();

    sys.refresh_all();

    let sys_disks = Disks::new_with_refreshed_list();

    println!("Called");
    let disks: Vec<Volume> = sys_disks
        .iter()
        .map(Volume::from)
        .filter(|v| is_user_facing_volume(&v.name, &v.mount_point.to_string_lossy()))
        .collect();

    Ok(disks)
}

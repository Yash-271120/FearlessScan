mod volume;

pub use volume::{get_volumes, DirectoryPath};

pub const fn bytes_to_gb(bytes: u64) -> u16 {
    (bytes / (1e+9 as u64)) as u16
}

pub fn is_user_facing_volume(_name: &str, mount_point: &str) -> bool {
    let platform = std::env::consts::OS;

    match platform {
        "windows" => {
            // Show drives like C:\, D:\ (ignore weird UNC or temp)
            mount_point
                .chars()
                .next()
                .map(|c| c.is_ascii_alphabetic())
                .unwrap_or(false)
                && mount_point.contains(":\\")
        }
        "linux" => {
            // Ignore docker, wsl, etc.
            let system_paths = [
                "/mnt/wsl",
                "/mnt/wslg",
                "/mnt/wslg",
                "/snap",
                "/proc",
                "/sys",
                "/dev",
                "/run",
                "/tmp",
            ];

            !system_paths.iter().any(|p| mount_point.starts_with(p))
                && (mount_point == "/"
                    || mount_point.starts_with("/mnt/")
                    || mount_point.starts_with("/home"))
        }
        "macos" => {
            // On macOS, volumes are often in /System/Volumes or /Volumes/XYZ
            mount_point == "/" || mount_point.starts_with("/Volumes/")
        }
        _ => false,
    }
}

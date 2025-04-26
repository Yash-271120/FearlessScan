use std::{
    fs::{self, File},
    io::{BufReader, Write},
    sync::{Arc, MutexGuard},
    time::Duration,
};

use once_cell::sync::Lazy;
use tokio::time;

use crate::{MyState, SafeMyState};

pub static CACHE_FILE_PATH: Lazy<String> = Lazy::new(|| {
    let mut cache_path = dirs::cache_dir().expect("Failed to get user cache dir");
    cache_path.push(format!("{}.cache.bin", env!("CARGO_PKG_NAME")));
    cache_path.to_string_lossy().to_string()
});

pub fn load_storage_cache(state_mux: &SafeMyState) -> bool {
    let mut state = state_mux.lock().unwrap();

    let cache_file = File::open(CACHE_FILE_PATH.as_str()).expect("Failed to open cache file");
    let buf_reader = BufReader::new(cache_file);

    if let Ok(decompressed) = zstd::decode_all(buf_reader) {
        let deserialized = serde_bencode::from_bytes(&decompressed);
        if let Ok(deserialized) = deserialized {
            state.storage_cache = deserialized;
            return true;
        }
    }

    println!("Failed to load the cache");

    return false;
}

pub fn save_to_file(mutex_guard: &MutexGuard<MyState>) {
    let serialized_cache = serde_bencode::to_string(&mutex_guard.storage_cache).unwrap();

    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(CACHE_FILE_PATH.as_str())
        .unwrap();

    let buffer =
        zstd::encode_all(serialized_cache.as_bytes(), 0).expect("Failed to compress cache.");
    file.write_all(&buffer).unwrap();
}

pub fn save_storage_cache(state_mux: &SafeMyState) {
    let state = state_mux.lock().unwrap();
    save_to_file(&state);
}

pub fn run_cache_poll(state_mux: &SafeMyState) {
    let state_clone = Arc::clone(state_mux);

    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(60));

        loop {
            interval.tick().await;

            let guard = state_clone.lock().unwrap();
            save_to_file(&guard);
        }
    });
}

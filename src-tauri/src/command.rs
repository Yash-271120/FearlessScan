use std::sync::Mutex;

use tauri::State;

use crate::MyState;

#[tauri::command]
pub fn yash(name: &str) -> String {
    format!("Yash says salam to {}", name)
}

#[tauri::command]
pub fn increase_counter(curr_state: State<'_, Mutex<MyState>>, add: u8) {
    let mut state = curr_state.lock().unwrap();
    state.counter += add;
}

#[tauri::command]
pub fn show_counter(curr_state: State<'_, Mutex<MyState>>) -> u8 {
    curr_state.lock().unwrap().counter
}

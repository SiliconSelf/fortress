#![doc = include_str!("../README.md")]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // This is inside of a macro. I can't do anything about it.
    #[allow(clippy::str_to_string)]
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

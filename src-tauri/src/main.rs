#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use tauri::api::cli::get_matches;

#[tauri::command]
fn printToJS() {
    println!("I was invoked from JS!");
}
fn main() {
  
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![printToJS])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
    
}

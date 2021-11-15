#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
fn print_to_js() {
    println!("I was invoked from JS!");
}
fn main() {
  
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![print_to_js])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
    
}

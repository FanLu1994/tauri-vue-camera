// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn close(){
    // 随便打印一句话
    println!("close the window");
    std::process::exit(0);
}
use tauri::Manager;

// 打开label为second的窗口,show改为true
#[tauri::command]
fn open_second_window(window: tauri::Window){
    let win = window.get_window("second").unwrap();
    if win.is_visible().unwrap() {
            win.hide().unwrap();
        } else {
            win.show().unwrap();
        }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,close,open_second_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

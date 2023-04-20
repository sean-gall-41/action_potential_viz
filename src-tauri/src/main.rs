// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn plot_logistic_map(a: f32, b: f32, delta: f32, r: f32) -> (Vec<f32>, Vec<f32>) {
    let mut res = (vec![], vec![]);
    let mut x = a;
    while x <= b {
        res.0.push(x);
        res.1.push(logistic_map(r, x));
        x += delta;
    }
    res
}
fn logistic_map(r: f32, x: f32) -> f32 {
    r * x * (1.0 - x)
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, plot_logistic_map])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

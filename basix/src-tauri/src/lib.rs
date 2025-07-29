mod commands;
mod events;
use commands::*;
use events::*;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(webview_window: tauri::WebviewWindow, name: &str) -> String {
    println!("WebviewWindow: {}", webview_window.label());
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(debug_assertions)] // only enable instrumentation in development builds
    // let devtools = tauri_plugin_devtools::init();
    let builder = tauri::Builder::default();

    // #[cfg(debug_assertions)]
    // {
    //     builder = builder.plugin(devtools);
    // }

    builder
        .manage(MyState::default())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_user, login, download])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use tauri::{
    menu::{Menu, MenuItem, MenuBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default();

    let tray_builder_clouser = |app: &mut tauri::App| {
        let quit = MenuItem::new(app, "quit", true, None::<&str>)?;
        let show = MenuItem::new(app, "show", true, None::<&str>)?;

        let tray_menu = Menu::with_items(app, &[&quit, &show])?;

        let _tray = TrayIconBuilder::new()
            .menu(&tray_menu)
            .show_menu_on_left_click(false)
            .on_menu_event(|app, event| match event.id.as_ref() {
                "1" => {
                    println!("quit menu item was clicked");
                    app.exit(0);
                }
                "2" => if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                },
                _ => {
                    println!("menu item {:?} not handled", event.id);
                }
            })
            .on_tray_icon_event(|tray, event| match event {
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } => {
                    println!("left click pressed and released");
                    let app = tray.app_handle();
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                _ => {
                    println!("unhandled event {event:?}");
                }
            })
            .icon(app.default_window_icon().unwrap().clone())
            .build(app)?;

        Ok(())
    };

    builder
        .setup(tray_builder_clouser)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

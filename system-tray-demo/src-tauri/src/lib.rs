// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#![allow(unused_variables, unused_imports)]

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command(async)]
async fn start_monitoring(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
    if let Err(error) = listen(move |event| {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        match event.event_type {
            EventType::MouseMove { x, y } => {
                println!("[{}] Mouse Move - x: {}, y: {}", timestamp, x, y);
            }
            EventType::ButtonPress(button) => {
                println!("[{}] Mouse Press - button: {:?}", timestamp, button);
            }
            EventType::ButtonRelease(button) => {
                println!("[{}] Mouse Release - button: {:?}", timestamp, button);
            }
            EventType::KeyPress(key) => {
                println!("[{}] Key Press - key: {:?}", timestamp, key);
            }
            EventType::KeyRelease(key) => {
                println!("[{}] Key Release - key: {:?}", timestamp, key);
            }
            EventType::Wheel { delta_x, delta_y } => {
                println!(
                    "[{}] Wheel - delta_x: {}, delta_y: {}",
                    timestamp, delta_x, delta_y
                );
            }
        }
    }) {
        println!("Error: {:?}", error);
    }

    Ok(())
}

use rdev::{listen, Button, Event, EventType, Key};
use std::{
    error::Error,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};
use tauri::{
    menu::{CheckMenuItem, IconMenuItem, Menu, MenuBuilder, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default();

    let tray_builder_clouser = |app: &mut tauri::App| {
        // let icon1 = tauri::image::Image::new(&[30, 30, 30], 32, 32);
        // let menu = MenuBuilder::new(app)
        //     .item(&MenuItem::new(app, "MenuItem 1", true, None::<&str>)?)
        //     .items(&[
        //         &CheckMenuItem::new(app, "CheckMenuItem 1", true, true, None::<&str>)?,
        //         // &IconMenuItem::new(app, "IconMenuItem 1", true, Some(icon1), None::<&str>)?,
        //     ])
        //     .separator()
        //     .cut()
        //     .copy()
        //     .paste()
        //     .separator()
        //     .text("item2", "MenuItem 2")
        //     .check("checkitem2", "CheckMenuItem 2")
        //     .icon(
        //         "iconitem2",
        //         "IconMenuItem 2",
        //         app.default_window_icon().cloned().unwrap(),
        //     )
        //     .build()?;

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
                "2" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
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
        .invoke_handler(tauri::generate_handler![greet, start_monitoring])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

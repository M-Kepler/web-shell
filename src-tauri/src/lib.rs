// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{menu::MenuBuilder, tray::TrayIconBuilder, Manager, WindowEvent};
use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};
use tauri::image::Image;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            // 创建菜单
            let menu = MenuBuilder::new(app)
                .text("show", "主窗口（Alt+Space）")
                .text("quit", "退出")
                .build()?;

            // 创建托盘
            TrayIconBuilder::with_id("tray")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| match event.id().0.as_str() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;

            // 让主窗口居中
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.center();
                let _ = window.hide();
            }

            // 注册 alt+space 全局快捷键
            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_shortcuts(["alt+space"])?
                    .with_handler(|app, shortcut, event| {
                        if event.state == ShortcutState::Pressed {
                            if shortcut.matches(Modifiers::ALT, Code::Space) {
                                if let Some(window) = app.get_webview_window("main") {
                                    if let Ok(visible) = window.is_visible() {
                                        if visible {
                                            let _ = window.hide();
                                        } else {
                                            let _ = window.show();
                                            let _ = window.set_focus();
                                        }
                                    }
                                }
                            }
                        }
                    })
                    .build(),
            )?;

            Ok(())
        })
        .on_window_event(|app, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.hide();
                }
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

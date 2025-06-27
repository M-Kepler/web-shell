// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{menu::MenuBuilder, tray::TrayIconBuilder, Manager, WindowEvent};
use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};
use tauri::image::Image;
use tauri_plugin_autostart;
use tauri_plugin_autostart::ManagerExt;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let autostart_manager = app.autolaunch();
            let enabled = autostart_manager.is_enabled().unwrap_or(false);

            // 动态菜单项
            let autostart_text = if enabled { "取消开机自启" } else { "开机自启" };

            let menu = MenuBuilder::new(app)
                .text("show", "主窗口（Alt+Space）")
                .text("autostart", autostart_text)
                .text("quit", "退出")
                .build()?;

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
                    "autostart" => {
                        let autostart_manager = app.autolaunch();
                        let enabled = autostart_manager.is_enabled().unwrap_or(false);
                        if enabled {
                            let _ = autostart_manager.disable();
                        } else {
                            let _ = autostart_manager.enable();
                        }
                        // 刷新菜单文本
                        let new_text = if enabled { "开机自启" } else { "取消开机自启" };
                        if let Some(tray) = app.tray_by_id("tray") {
                            let _ = tray.set_menu(
                                Some(MenuBuilder::new(app)
                                    .text("show", "主窗口（Alt+Space）")
                                    .text("autostart", new_text)
                                    .text("quit", "退出")
                                    .build().unwrap())
                            );
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;

            // 让主窗口居中并隐藏
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.center();
                // let _ = window.hide(); // 移除隐藏主窗口
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

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{menu::MenuBuilder, tray::TrayIconBuilder, Manager, WindowEvent};
use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};
use tauri::image::Image;
use tauri_plugin_autostart;
use tauri_plugin_autostart::ManagerExt;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Config {
    app: AppConfig,
    window: WindowConfig,
    web: WebConfig,
    hotkey: HotkeyConfig,
    autostart: AutostartConfig,
    tray: TrayConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AppConfig {
    name: String,
    version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct WindowConfig {
    title: String,
    width: u32,
    height: u32,
    resizable: bool,
    fullscreen: bool,
    decorations: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct WebConfig {
    url: String,
    title: String,
    allow: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct HotkeyConfig {
    show: String,
    hide: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AutostartConfig {
    enabled: bool,
    prompt_on_first_run: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TrayConfig {
    enabled: bool,
    menu: MenuConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct MenuConfig {
    show: String,
    autostart: String,
    autostart_disable: String,
    quit: String,
}

// 默认配置
fn default_config() -> Config {
    Config {
        app: AppConfig {
            name: "Web Shell".to_string(),
            version: "1.0.0".to_string(),
        },
        window: WindowConfig {
            title: "Web Shell".to_string(),
            width: 400,
            height: 700,
            resizable: true,
            fullscreen: false,
            decorations: false,
        },
        web: WebConfig {
            url: "https://www.kimi.com/".to_string(),
            title: "Kimi Chat".to_string(),
            allow: "clipboard-write; clipboard-read;".to_string(),
        },
        hotkey: HotkeyConfig {
            show: "Alt+Space".to_string(),
            hide: "Alt+Space".to_string(),
        },
        autostart: AutostartConfig {
            enabled: false,
            prompt_on_first_run: true,
        },
        tray: TrayConfig {
            enabled: true,
            menu: MenuConfig {
                show: "显示窗口".to_string(),
                autostart: "开机自启".to_string(),
                autostart_disable: "取消开机自启".to_string(),
                quit: "退出".to_string(),
            },
        },
    }
}

// 读取配置文件
fn load_config(config_dir: &PathBuf) -> Config {
    let config_path = config_dir.join("config.json");
    
    if let Ok(content) = fs::read_to_string(&config_path) {
        if let Ok(config) = serde_json::from_str::<Config>(&content) {
            return config;
        }
    }
    
    // 如果配置文件不存在或解析失败，创建默认配置文件
    let default_config = default_config();
    if let Ok(content) = serde_json::to_string_pretty(&default_config) {
        let _ = fs::write(&config_path, content);
    }
    
    default_config
}

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
            // 读取配置文件
            let config_dir = app.path().app_config_dir().unwrap();
            let config = load_config(&config_dir);
            
            // 更新窗口标题
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_title(&config.window.title);
            }

            let autostart_manager = app.autolaunch();
            let enabled = autostart_manager.is_enabled().unwrap_or(false);

            // 动态菜单项
            let autostart_text = if enabled {
                config.tray.menu.autostart_disable.clone()
            } else {
                config.tray.menu.autostart.clone()
            };

            let menu = MenuBuilder::new(app)
                .text("show", &config.tray.menu.show)
                .text("autostart", &autostart_text)
                .text("quit", &config.tray.menu.quit)
                .build()?;

            TrayIconBuilder::with_id("tray")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| {
                    // 重新读取配置
                    let config_dir = app.path().app_config_dir().unwrap();
                    let config = load_config(&config_dir);
                    
                    match event.id().0.as_str() {
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
                            let new_text = if enabled {
                                config.tray.menu.autostart.clone()
                            } else {
                                config.tray.menu.autostart_disable.clone()
                            };
                            if let Some(tray) = app.tray_by_id("tray") {
                                let _ = tray.set_menu(
                                    Some(MenuBuilder::new(app)
                                        .text("show", &config.tray.menu.show)
                                        .text("autostart", &new_text)
                                        .text("quit", &config.tray.menu.quit)
                                        .build().unwrap())
                                );
                            }
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .build(app)?;

            // 让主窗口居中并隐藏
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.center();
                // let _ = window.hide(); // 移除隐藏主窗口
            }

            // 注册全局快捷键
            let hotkey_str = config.hotkey.show.clone();
            let hotkey_lower = hotkey_str.to_lowercase();
            
            // 解析快捷键（简化版，支持 Alt+Space）
            let shortcut = if hotkey_lower.contains("alt+space") {
                Some(["alt+space"])
            } else {
                None
            };

            if let Some(shortcuts) = shortcut {
                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_shortcuts(&shortcuts)?
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
            }

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

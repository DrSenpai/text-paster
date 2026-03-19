// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use arboard::Clipboard;
use tauri::{Emitter, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};
use std::sync::Mutex;
use tauri::tray::TrayIconBuilder;
use std::thread;
use std::time::Duration;

static MINIMIZE_TO_TRAY: Mutex<bool> = Mutex::new(false);

#[cfg(not(target_os = "windows"))]
use enigo::{Enigo, Key, KeyboardControllable};

#[cfg(target_os = "windows")]
fn simulate_type(text: &str) {
    use std::mem::zeroed;
    use winapi::um::winuser::{
        INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE, SendInput,
    };

    unsafe {
        for c in text.chars() {
            // Key down
            let mut input: INPUT = zeroed();
            input.type_ = INPUT_KEYBOARD;
            *input.u.ki_mut() = KEYBDINPUT {
                wVk: 0,
                wScan: c as u16,
                dwFlags: KEYEVENTF_UNICODE,
                time: 0,
                dwExtraInfo: 0,
            };
            SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);

            thread::sleep(Duration::from_millis(5));

            // Key up
            let mut input: INPUT = zeroed();
            input.type_ = INPUT_KEYBOARD;
            *input.u.ki_mut() = KEYBDINPUT {
                wVk: 0,
                wScan: c as u16,
                dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                time: 0,
                dwExtraInfo: 0,
            };
            SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn simulate_type(text: &str) {
    let mut enigo = Enigo::new();
    for c in text.chars() {
        enigo.key_click(Key::Layout(c));
        thread::sleep(Duration::from_millis(5));
    }
}

#[tauri::command]
fn paste_text(text: String, auto_paste: bool) -> Result<String, String> {
    if text.trim().is_empty() {
        return Err("Text darf nicht leer sein".into());
    }

    let mut clipboard = Clipboard::new().map_err(|e| format!("Clipboard error: {}", e))?;

    clipboard
        .set_text(text.clone())
        .map_err(|e| format!("Clipboard error: {}", e))?;

    if auto_paste {
        thread::sleep(Duration::from_millis(40));
        simulate_type(&text);
        Ok("Text erfolgreich eingefügt!".into())
    } else {
        Ok("Text in Zwischenablage kopiert.".into())
    }
}

#[tauri::command]
fn register_hotkeys(app: tauri::AppHandle, hotkey_mode: String) -> Result<(), String> {
    let global_shortcut = app.global_shortcut();

    let _ = global_shortcut.unregister_all();

    let shortcuts = if hotkey_mode == "numpad" {
        vec![
            ("Numpad1", "preset1"),
            ("Numpad2", "preset2"),
            ("Numpad3", "preset3"),
            ("Numpad4", "preset4"),
            ("Numpad5", "preset5"),
        ]
    } else {
        vec![
            ("Ctrl+1", "preset1"),
            ("Ctrl+2", "preset2"),
            ("Ctrl+3", "preset3"),
            ("Ctrl+4", "preset4"),
            ("Ctrl+5", "preset5"),
        ]
    };

    for (shortcut_str, preset_key) in shortcuts {
        let shortcut: tauri_plugin_global_shortcut::Shortcut = match shortcut_str.try_into() {
            Ok(s) => s,
            Err(e) => {
                println!("Invalid shortcut {}: {:?}", shortcut_str, e);
                continue;
            }
        };

        let preset_key_clone = preset_key.to_string();

        if let Err(e) = global_shortcut.on_shortcut(shortcut, move |app, _shortcut, event| {
            // FIX: verhindert doppeltes Auslösen
            if event.state() != ShortcutState::Pressed {
                return;
            }

            let _ = app.emit("paste-preset", preset_key_clone.clone());
        }) {
            println!("Warning: Failed to register shortcut {}: {:?}", shortcut_str, e);
        }
    }

    Ok(())
}

#[tauri::command]
fn set_minimize_to_tray(enabled: bool) -> Result<(), String> {
    *MINIMIZE_TO_TRAY.lock().map_err(|e| format!("Mutex error: {}", e))? = enabled;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            paste_text,
            register_hotkeys,
            set_minimize_to_tray
        ])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            let window_clone = window.clone();

            window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    let minimize_to_tray = *MINIMIZE_TO_TRAY.lock().unwrap();
                    if minimize_to_tray {
                        window_clone.hide().unwrap();
                        api.prevent_close();
                    }
                }
            });

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("Text Paster")
                .on_tray_icon_event(|tray, event| {
                    if let tauri::tray::TrayIconEvent::Click { .. } = event {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
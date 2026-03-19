// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use arboard::Clipboard;
use enigo::{Enigo, Key, KeyboardControllable};
use tauri::Emitter;
use tauri_plugin_global_shortcut::GlobalShortcutExt;

#[tauri::command]
fn paste_text(text: String) -> Result<String, String> {
    if text.trim().is_empty() {
        return Err("Text darf nicht leer sein".into());
    }

    // Copy text to the system clipboard.
    let mut clipboard = Clipboard::new().map_err(|e| format!("Clipboard error: {}", e))?;
    clipboard
        .set_text(text.clone())
        .map_err(|e| format!("Clipboard error: {}", e))?;

    // Simulate a paste shortcut so the text is inserted at the current cursor position.
    // Uses Ctrl+V on Windows/Linux and ⌘+V on macOS.
    let mut enigo = Enigo::new();
    let modifier = if cfg!(target_os = "macos") {
        Key::Meta
    } else {
        Key::Control
    };

    enigo.key_down(modifier);
    enigo.key_click(Key::Layout('v'));
    enigo.key_up(modifier);

    Ok("Text erfolgreich eingefügt!".into())
}

#[tauri::command]
fn register_hotkeys(app: tauri::AppHandle, hotkey_mode: String) -> Result<(), String> {
    let global_shortcut = app.global_shortcut();

    // Unregister existing hotkeys first
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

        if let Err(e) = global_shortcut.on_shortcut(shortcut, move |app, _shortcut, _event| {
            let _ = app.emit("paste-preset", preset_key_clone.clone());
        }) {
            // Log the error but don't fail - hotkeys might already be registered
            println!("Warning: Failed to register shortcut {}: {:?}", shortcut_str, e);
        }
    }

    Ok(())
}

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_global_shortcut::Builder::new().build())
    .invoke_handler(tauri::generate_handler![paste_text, register_hotkeys])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
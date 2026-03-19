// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use arboard::Clipboard;
use tauri::{Emitter, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};
use std::sync::Mutex;
use tauri::tray::TrayIconBuilder;
use std::thread;
use std::time::Duration;

/// Globaler State für die "Minimize to Tray" Einstellung
/// Wird vom Frontend aus kontrolliert und bestimmt, ob das Fenster
/// beim Schließen in den System Tray minimiert oder beendet wird
static MINIMIZE_TO_TRAY: Mutex<bool> = Mutex::new(false);

#[cfg(not(target_os = "windows"))]
use enigo::{Enigo, Key, KeyboardControllable};

/// Windows-spezifische Implementierung für Character-by-Character Typing
/// Verwendet die Windows API (SendInput mit KEYEVENTF_UNICODE), um jeden Character
/// einzeln zu simulieren. Dies umgeht Anti-Paste-Mechaniken in Chat-Anwendungen.
#[cfg(target_os = "windows")]
fn simulate_type(text: &str) {
    use std::mem::zeroed;
    use winapi::um::winuser::{
        INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE, SendInput,
    };

    unsafe {
        for c in text.chars() {
            // Simuliere Tasten-Druck (Key down)
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

            // Kleine Verzögerung für realistisches Typing
            thread::sleep(Duration::from_millis(5));

            // Simuliere Tasten-Loslassen (Key up)
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

/// macOS/Linux Implementierung für Character-by-Character Typing
/// Nutzt die enigo Bibliothek für plattformunabhängiges Tastatur-Input
#[cfg(not(target_os = "windows"))]
fn simulate_type(text: &str) {
    let mut enigo = Enigo::new();
    for c in text.chars() {
        // Tippe jeden Character einzeln mit 5ms Verzögerung
        enigo.key_click(Key::Layout(c));
        thread::sleep(Duration::from_millis(5));
    }
}

/// Tauri Command zum Einfügen/Tippen von Text
/// 
/// Parameter:
/// - text: Der Text, der eingefügt werden soll
/// - auto_paste: Wenn true, wird der Text automatisch getippt; wenn false, nur in Zwischenablage kopiert
///
/// Verhalten:
/// 1. Validiert, dass der Text nicht leer ist
/// 2. Kopiert Text in die Systemzwischenablage (Fallback)
/// 3. Wenn auto_paste, tippt den Text Zeichen für Zeichen
#[tauri::command]
fn paste_text(text: String, auto_paste: bool) -> Result<String, String> {
    // Verhindere Einfügen von leerem Text
    if text.trim().is_empty() {
        return Err("Text darf nicht leer sein".into());
    }

    // Kopiere Text in Zwischenablage als Fallback
    let mut clipboard = Clipboard::new().map_err(|e| format!("Clipboard error: {}", e))?;
    clipboard
        .set_text(text.clone())
        .map_err(|e| format!("Clipboard error: {}", e))?;

    // Auto-Paste: Tippe den Text Zeichen für Zeichen
    if auto_paste {
        // Kurze Verzögerung zum Sicherstellen, dass das Ziel-Fenster fokussiert ist
        thread::sleep(Duration::from_millis(40));
        simulate_type(&text);
        Ok("Text erfolgreich eingefügt!".into())
    } else {
        // Nur in Zwischenablage kopiert, ohne zu tippen
        Ok("Text in Zwischenablage kopiert.".into())
    }
}

/// Tauri Command zum Registrieren von globalen Hotkeys
/// 
/// Supports zwei Hotkey-Modi:
/// - "numpad": Numpad 1-5 für Presets
/// - "ctrl": Ctrl+1 bis Ctrl+5 für Presets
/// 
/// Wenn ein Hotkey gedrückt wird, emittiert diese Funktion ein "paste-preset" Event
/// an das Frontend mit dem Preset-Schlüssel (preset1, preset2, etc.)
#[tauri::command]
fn register_hotkeys(app: tauri::AppHandle, hotkey_mode: String) -> Result<(), String> {
    let global_shortcut = app.global_shortcut();

    // Deregistrière alle existierenden Hotkeys (für Mode-Wechsel)
    let _ = global_shortcut.unregister_all();

    // Wähle die Hotkey-Kombinationen basierend auf dem Modus
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

    // Registriere jeden Hotkey
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
            // Nur auf Key-Down reagieren, nicht auf Key-Up (verhindert doppeltes Auslösen)
            if event.state() != ShortcutState::Pressed {
                return;
            }

            // Emittiere Event an Frontend mit dem Preset-Schlüssel
            let _ = app.emit("paste-preset", preset_key_clone.clone());
        }) {
            println!("Warning: Failed to register shortcut {}: {:?}", shortcut_str, e);
        }
    }

    Ok(())
}

/// Tauri Command zum Einstellen des "Minimize to Tray" Verhaltens
/// 
/// Wenn enabled=true:
/// - Fenster wird beim Schließen in System Tray minimiert
/// - App wird nicht beendet
/// 
/// Wenn enabled=false:
/// - Fenster wird normal geschlossen und App beendet
#[tauri::command]
fn set_minimize_to_tray(enabled: bool) -> Result<(), String> {
    // Speichere die Einstellung im statischen State (wird beim App-Start aus localStorage wieder geladen)
    *MINIMIZE_TO_TRAY.lock().map_err(|e| format!("Mutex error: {}", e))? = enabled;
    Ok(())
}

/// Haupteinstiegspunkt der Tauri-Anwendung
/// 
/// Initialisiert:
/// 1. Global Shortcut Plugin für Hotkey-Handling
/// 2. Invoke Handler für Frontend-Kommunikation
/// 3. Window Event Handler für Minimize-to-Tray Verhalten
/// 4. System Tray Icon mit Click-Handler
fn main() {
    tauri::Builder::default()
        // Registriere das Global Shortcut Plugin
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        // Expose Rust Commands an das Frontend JavaScript
        .invoke_handler(tauri::generate_handler![
            paste_text,
            register_hotkeys,
            set_minimize_to_tray
        ])
        // Setup-Handler für Fenster- und Tray-Konfiguration
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            let window_clone = window.clone();

            // Listener für Fenster-Events (z.B. Close Button Klick)
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    // Wenn Minimize to Tray aktiviert ist, verstecke das Fenster statt es zu schließen
                    let minimize_to_tray = *MINIMIZE_TO_TRAY.lock().unwrap();
                    if minimize_to_tray {
                        window_clone.hide().unwrap();
                        api.prevent_close(); // Verhindere das Schließen der App
                    }
                }
            });

            // Erstelle System Tray Icon
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("Text Paster") // Zeige Namen beim Hover
                .on_tray_icon_event(|tray, event| {
                    // Auf Tray-Icon Klick: Fenster zeigen und fokussieren
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
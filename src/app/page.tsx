/// Haupt-Seite von Text Paster
/// Zeigt 5 Text-Input-Felder (Presets) zur Konfiguration und verwaltung von Shortcuts
/// Unterstützt Hotkey-Triggered Pastes und unterschiedliche Paste-Modi

"use client";

import { useEffect, useMemo, useState } from "react";
import { useTranslation } from "./hooks/useTranslation";

// Definierte Typen für die verschiedenen Konfigurationsmöglichkeiten
type HotkeyMode = "numpad" | "ctrl";
type PasteMode = "auto" | "clipboard";

// LocalStorage Keys für Persistierung der Einstellungen
const STORAGE_KEY_HOTKEY = "textPaster.hotkeyMode";
const STORAGE_KEY_PASTE_MODE = "textPaster.pasteMode";
const STORAGE_KEY_TRAY = "textPaster.minimizeToTray";

export default function Home() {
  const { t, language } = useTranslation();

  // State für die 5 Preset-Eingabefelder
  const [inputs, setInputs] = useState(["", "", "", "", ""]);

  // State für die Hotkey-Konfiguration (Numpad oder Ctrl+1-5)
  const [hotkeyMode, setHotkeyMode] = useState<HotkeyMode>("numpad");

  // State für den Paste-Modus (Auto-Tippen oder nur Zwischenablage)
  const [pasteMode, setPasteMode] = useState<PasteMode>("auto");

  // Beim Laden: Lade gespeicherte Einstellungen und Presets aus localStorage
  useEffect(() => {
    // Lade Hotkey-Modus
    const storedHotkey = window.localStorage.getItem(STORAGE_KEY_HOTKEY) as HotkeyMode | null;
    const initialMode = (storedHotkey === "numpad" || storedHotkey === "ctrl") ? storedHotkey : "numpad";
    setHotkeyMode(initialMode);

    // Lade Paste-Modus
    const storedPasteMode = window.localStorage.getItem(STORAGE_KEY_PASTE_MODE) as PasteMode | null;
    const initialPasteMode = storedPasteMode === "clipboard" ? "clipboard" : "auto";
    setPasteMode(initialPasteMode);

    // Lade minimizeToTray Einstellung und wende sie auf Rust-Seite an
    const storedTrayMode = window.localStorage.getItem(STORAGE_KEY_TRAY);
    const initialTrayEnabled = storedTrayMode === "enabled";

    // Lade gespeicherte Presets aus localStorage
    const storedPresets = window.localStorage.getItem("textPaster.presets");
    if (storedPresets) {
      try {
        const parsed = JSON.parse(storedPresets);
        if (Array.isArray(parsed) && parsed.length === 5) {
          setInputs(parsed);
        }
      } catch (e) {
        console.error("Failed to parse stored presets:", e);
      }
    }

    // Registriere Hotkeys beim App-Start und wende Tray-Einstellung an
    const registerHotkeys = async () => {
      try {
        const { invoke } = await import("@tauri-apps/api/core");
        await invoke("register_hotkeys", { hotkeyMode: initialMode });
        await invoke("set_minimize_to_tray", { enabled: initialTrayEnabled });
      } catch (e) {
        console.error("Failed to register hotkeys or set tray behavior:", e);
      }
    };

    registerHotkeys();
  }, []);

  // Listener für Hotkey-Events vom Backend
  // Wenn ein registrierter Hotkey (Numpad oder Ctrl+1-5) gedrückt wird,
  // wird das entsprechende Preset automatisch eingefügt
  useEffect(() => {
    const handlePastePreset = async (event: any) => {
      // Extrahiere die Preset-Nummer aus dem Event (z.B. "preset1" -> 0)
      const presetIndex = parseInt(event.payload.replace("preset", "")) - 1;

      // Validiere und füge das Preset ein
      if (presetIndex >= 0 && presetIndex < inputs.length && inputs[presetIndex].trim()) {
        try {
          const { invoke } = await import("@tauri-apps/api/core");
          await invoke("paste_text", {
            text: inputs[presetIndex],
            autoPaste: pasteMode === "auto",
          });
        } catch (e) {
          console.error("Failed to paste preset:", e);
        }
      }
    };

    const setupListener = async () => {
      try {
        const { listen } = await import("@tauri-apps/api/event");
        const unlisten = await listen("paste-preset", handlePastePreset);
        return unlisten;
      } catch (e) {
        console.error("Failed to setup event listener:", e);
      }
    };

    let unlisten: (() => void) | undefined;
    setupListener().then((u) => (unlisten = u));

    // Cleanup: Entferne Listener beim Unmount
    return () => {
      if (unlisten) unlisten();
    };
  }, [inputs]);

  // Generiere die Hotkey-Hinweis-Beschreibung für die UI
  const hotkeyHint = useMemo(() => {
    return hotkeyMode === "numpad"
      ? t("numpadOption")
      : t("ctrlOption");
  }, [hotkeyMode, t]);

  // Handler für Änderungen an den Presets
  // Aktualisiert den State und speichert die Presets in localStorage
  const handleChange = (index: number, value: string) => {
    const newInputs = [...inputs];
    newInputs[index] = value;
    setInputs(newInputs);

    // Persistiere Presets
    window.localStorage.setItem("textPaster.presets", JSON.stringify(newInputs));
  };

  // Handler für den "Save" Button
  // Fügt alle Presets zusammen in das Clipboard ein (getrennt durch |)
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    const { invoke } = await import("@tauri-apps/api/core");

    console.log("Inputs:", inputs);

    // Verbinde alle Presets mit " | " und füge sie ein
    await invoke("paste_text", {
      text: inputs.join(" | "),
      autoPaste: pasteMode === "auto",
    });
  };

  return (
    <div className="w-screen h-screen flex items-center justify-center bg-gray-100 dark:bg-black p-6">
      <form
        onSubmit={handleSubmit}
        className="w-full max-w-lg  p-6 rounded-3xl flex flex-col gap-4"
      >
        <header className="flex flex-col sm:flex-row sm:justify-between sm:items-start gap-2">
          <div>
            <h1 className="text-2xl font-semibold text-slate-900 dark:text-white font-mono">
              {t("title")}
            </h1>
            <p className="text-md text-slate-600 dark:text-slate-300 font-mono">
              {t("languageLabel")}: <span className="font-medium font-mono">{language === "de" ? "Deutsch" : "English"}</span> · {t("hotkeysLabel")}: <span className="font-medium font-mono">{hotkeyHint}</span> · {t("pasteModeSection")}: <span className="font-medium font-mono">{pasteMode === "auto" ? t("pasteModeAuto") : t("pasteModeClipboard")}</span>
            </p>
            <p className="text-sm text-slate-500 dark:text-slate-400 font-mono">
              {t("settingsHint")}
            </p>
          </div>

        </header>
        {inputs.map((value, i) => (
          <input
            key={i}
            type="text"
            value={value}
            onChange={(e) => handleChange(i, e.target.value)}
            placeholder={`${t("presetPlaceholder")} ${i + 1}`}
            className="
              w-full
              p-3
              rounded-2xl
              border border-gray-300 dark:border-zinc-700
              bg-white dark:bg-zinc-800 shadow-md dark:shadow-zinc-900
              text-black dark:text-white font-mono
              placeholder-gray-400 dark:placeholder-gray-300
              text-base
              transition-transform duration-200
              hover:scale-105
              focus:outline-none focus:ring-2 focus:ring-blue-500
            "
          />
        ))}

        <button
          type="submit"
          className="
            cursor-pointer
            mt-4
            p-3
            rounded-2xl
            text-base
            text-white font-mono
            bg-blue-600 dark:bg-zinc-600
            hover:bg-blue-700 dark:hover:bg-zinc-500
            transition-all duration-200
            hover:scale-105
          "
        >
          {t("saveButton")}
        </button>
      </form>
    </div>
  );
}
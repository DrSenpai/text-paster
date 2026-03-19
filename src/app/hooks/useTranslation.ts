/// Custom Hook für mehrsprachige Übersetzungen
/// Verwaltet die aktuelle Sprache und stellt eine Hilfsfunktion bereit,
/// um Übersetzungen aus einer vordefinierten Translations-Map zu holen.

"use client";

import { useEffect, useState } from "react";
import { translations, Language, TranslationKey } from "../translations";

// LocalStorage Key für die Spracheinstellung
const STORAGE_KEY_LANGUAGE = "textPaster.language";

export function useTranslation() {
    // State für die aktuelle Sprache (Standard: Deutsch)
    const [language, setLanguage] = useState<Language>("de");

    // Beim Laden: Lade die gespeicherte Sprache aus localStorage
    useEffect(() => {
        const stored = window.localStorage.getItem(STORAGE_KEY_LANGUAGE) as Language | null;
        if (stored === "de" || stored === "en") {
            setLanguage(stored);
        }
    }, []);

    /// Hilfsfunktion um einen Übersetzungs-Schlüssel in die aktuelle Sprache zu übersetzen
    const t = (key: TranslationKey): string => {
        return translations[language][key];
    };

    // Gebe die Übersetzungsfunktion, aktuelle Sprache, und Setter zurück
    return { t, language, setLanguage };
}

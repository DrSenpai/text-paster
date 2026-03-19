"use client";

import { useEffect, useState } from "react";
import { translations, Language, TranslationKey } from "../translations";

const STORAGE_KEY_LANGUAGE = "textPaster.language";

export function useTranslation() {
    const [language, setLanguage] = useState<Language>("de");

    useEffect(() => {
        const stored = window.localStorage.getItem(STORAGE_KEY_LANGUAGE) as Language | null;
        if (stored === "de" || stored === "en") {
            setLanguage(stored);
        }
    }, []);

    const t = (key: TranslationKey): string => {
        return translations[language][key];
    };

    return { t, language, setLanguage };
}

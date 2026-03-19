/* eslint react-hooks/set-state-in-effect: "off" */
"use client";

import { useEffect, useMemo, useState } from "react";
import { useRouter } from "next/navigation";
import { useTranslation } from "../hooks/useTranslation";
import { Language } from "../translations";

const STORAGE_KEY_HOTKEY = "textPaster.hotkeyMode";

type HotkeyMode = "numpad" | "ctrl";

const languageLabels = {
    de: "Deutsch",
    en: "English",
};

export default function SettingsPage() {
    const { t, language, setLanguage } = useTranslation();
    const router = useRouter();
    const [hotkeyMode, setHotkeyMode] = useState<HotkeyMode>("numpad");
    const [saved, setSaved] = useState(false);

    useEffect(() => {
        const storedHotkey = window.localStorage.getItem(STORAGE_KEY_HOTKEY) as HotkeyMode | null;
        if (storedHotkey) {
            setHotkeyMode(storedHotkey);
        }
    }, []);

    const saveSettings = () => {
        window.localStorage.setItem("textPaster.language", language);
        window.localStorage.setItem(STORAGE_KEY_HOTKEY, hotkeyMode);
        setSaved(true);

        // Register hotkeys
        const registerHotkeys = async () => {
            try {
                const { invoke } = await import("@tauri-apps/api/core");
                await invoke("register_hotkeys", { hotkeyMode });
            } catch (e) {
                console.error("Failed to register hotkeys:", e);
            }
        };

        registerHotkeys();

        window.setTimeout(() => setSaved(false), 2000);
    };

    const goBack = () => router.push("/");

    const hotkeyDescription = useMemo(() => {
        return hotkeyMode === "numpad"
            ? t("hotkeyDescriptionNumpad")
            : t("hotkeyDescriptionCtrl");
    }, [hotkeyMode, t]);

    return (
        <div className="w-screen h-screen flex flex-col items-center justify-center bg-gray-100 dark:bg-black p-6">
            <div className="w-full max-w-xl bg-white dark:bg-zinc-800 p-8 rounded-3xl shadow-xl dark:shadow-zinc-900 space-y-6">
                <header className="flex items-start justify-between gap-4">
                    <div>
                        <h1 className="text-2xl font-semibold text-slate-900 dark:text-white">
                            {t("settingsTitle")}
                        </h1>
                        <p className="mt-1 text-sm text-slate-600 dark:text-slate-300">
                            {t("settingsDescription")}
                        </p>
                    </div>
                    <button
                        type="button"
                        onClick={goBack}
                        className="rounded-full border border-slate-200 bg-white px-4 py-2 text-sm font-medium text-slate-700 hover:bg-slate-50 dark:border-zinc-700 dark:bg-zinc-800 dark:text-slate-100 dark:hover:bg-zinc-700"
                    >
                        {t("backButton")}
                    </button>
                </header>

                <section className="space-y-3">
                    <h2 className="text-lg font-semibold text-slate-900 dark:text-white">
                        {t("languageSection")}
                    </h2>
                    <div className="grid gap-2 md:grid-cols-2">
                        {(Object.keys(languageLabels) as Language[]).map((lang) => (
                            <label
                                key={lang}
                                className="flex cursor-pointer items-center gap-3 rounded-xl border border-slate-200 bg-white px-4 py-3 text-sm text-slate-800 shadow-sm transition hover:border-blue-400 dark:border-zinc-700 dark:bg-zinc-900 dark:text-slate-100"
                            >
                                <input
                                    type="radio"
                                    name="language"
                                    value={lang}
                                    checked={language === lang}
                                    onChange={() => setLanguage(lang)}
                                    className="h-4 w-4 accent-blue-600"
                                />
                                {languageLabels[lang]}
                            </label>
                        ))}
                    </div>
                </section>

                <section className="space-y-3">
                    <h2 className="text-lg font-semibold text-slate-900 dark:text-white">
                        {t("hotkeySection")}
                    </h2>
                    <div className="grid gap-2 md:grid-cols-2">
                        <label className="flex cursor-pointer items-center gap-3 rounded-xl border border-slate-200 bg-white px-4 py-3 text-sm text-slate-800 shadow-sm transition hover:border-blue-400 dark:border-zinc-700 dark:bg-zinc-900 dark:text-slate-100">
                            <input
                                type="radio"
                                name="hotkey"
                                value="numpad"
                                checked={hotkeyMode === "numpad"}
                                onChange={() => setHotkeyMode("numpad")}
                                className="h-4 w-4 accent-blue-600"
                            />
                            {t("numpadOption")}
                        </label>
                        <label className="flex cursor-pointer items-center gap-3 rounded-xl border border-slate-200 bg-white px-4 py-3 text-sm text-slate-800 shadow-sm transition hover:border-blue-400 dark:border-zinc-700 dark:bg-zinc-900 dark:text-slate-100">
                            <input
                                type="radio"
                                name="hotkey"
                                value="ctrl"
                                checked={hotkeyMode === "ctrl"}
                                onChange={() => setHotkeyMode("ctrl")}
                                className="h-4 w-4 accent-blue-600"
                            />
                            {t("ctrlOption")}
                        </label>
                    </div>
                    <p className="text-sm text-slate-600 dark:text-slate-300">{hotkeyDescription}</p>
                </section>

                <div className="flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between">
                    <button
                        type="button"
                        onClick={saveSettings}
                        className="cursor-pointer w-full rounded-2xl dark:bg-zinc-600 bg-blue-600 px-6 py-3 text-sm font-semibold text-white shadow-sm hover:bg-blue-700 dark:hover:bg-zinc-500 sm:w-auto"
                    >
                        {t("saveSettings")}
                    </button>
                    {saved && (
                        <div className="rounded-2xl bg-emerald-100 px-4 py-2 text-sm text-emerald-800 dark:bg-emerald-900 dark:text-emerald-200">
                            {t("savedMessage")}
                        </div>
                    )}
                </div>
            </div>
        </div>
    );
}

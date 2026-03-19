"use client";

import { useCallback } from "react";
import { useRouter, usePathname } from "next/navigation";

export default function SettingsFab() {
    const router = useRouter();
    const pathname = usePathname();

    const goToSettings = useCallback(() => {
        if (pathname !== "/settings") {
            router.push("/settings");
        }
    }, [pathname, router]);

    return (
        <button
            type="button"
            onClick={goToSettings}
            aria-label="Einstellungen"
            className="cursor-pointer fixed bottom-4 right-4 z-50 grid h-12 w-12 place-items-center rounded-full dark:bg-zinc-600 bg-white/90 text-xl shadow-lg shadow-black/10 ring-1 ring-white/40 backdrop-blur transition hover:bg-white dark:hover:bg-zinc-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-white/80"
        >
            <span className="text-2xl">⚙️</span>
        </button>
    );
}

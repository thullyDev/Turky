import { convertFileSrc } from "@tauri-apps/api/core";
import { useCallback, useEffect, useMemo, useState } from "react";

import {
    downloadCatalogFont,
    listDownloadedFonts,
    type DownloadedFontInfo,
} from "../api/font-api";
import { catalogFontById } from "../../entities/overlay/font-catalog";

export function useDownloadedFonts() {
    const [entries, setEntries] = useState<DownloadedFontInfo[]>([]);
    const [loadingId, setLoadingId] = useState<string | null>(null);
    const [error, setError] = useState("");

    const refresh = useCallback(async () => {
        try {
            const fonts = await listDownloadedFonts();
            setEntries(fonts);
        } catch {
            setEntries([]);
        }
    }, []);

    useEffect(() => {
        void refresh();
    }, [refresh]);

    const families = useMemo(() => {
        const map: Record<string, string> = {};

        for (const entry of entries) {
            map[entry.id] = entry.family;
        }

        return map;
    }, [entries]);

    useEffect(() => {
        const styleId = "turky-downloaded-fonts";

        let element = document.getElementById(styleId) as HTMLStyleElement | null;

        if (!element) {
            element = document.createElement("style");
            element.id = styleId;
            document.head.appendChild(element);
        }

        const rules = entries
            .map((entry) => {
                const src = convertFileSrc(entry.path);

                return `@font-face { font-family: "${entry.family}"; src: url("${src}") format("truetype"); font-display: swap; }`;
            })
            .join("\n");

        element.textContent = rules;
    }, [entries]);

    const downloadFont = useCallback(
        async (fontId: string) => {
            const catalog = catalogFontById(fontId);

            if (!catalog) {
                setError("Unknown font.");
                return false;
            }

            setLoadingId(fontId);
            setError("");

            try {
                const downloaded = await downloadCatalogFont(
                    catalog.id,
                    catalog.url,
                    catalog.family,
                );
                setEntries((current) => {
                    const without = current.filter((entry) => entry.id !== downloaded.id);

                    return [...without, downloaded];
                });

                return true;
            } catch (cause) {
                setError(String(cause));
                return false;
            } finally {
                setLoadingId(null);
            }
        },
        [],
    );

    const isDownloaded = useCallback(
        (fontId: string) => entries.some((entry) => entry.id === fontId),
        [entries],
    );

    return {
        families,
        downloadFont,
        isDownloaded,
        loadingId,
        error,
        refresh,
    };
}

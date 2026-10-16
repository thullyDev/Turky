import { invoke } from "@tauri-apps/api/core";

export interface DownloadedFontInfo {
    id: string;
    family: string;
    path: string;
}

export function listDownloadedFonts(): Promise<DownloadedFontInfo[]> {
    return invoke<DownloadedFontInfo[]>("list_downloaded_fonts");
}

export function downloadCatalogFont(
    fontId: string,
    url: string,
    family: string,
): Promise<DownloadedFontInfo> {
    return invoke<DownloadedFontInfo>("download_font", { fontId, url, family });
}

import { invoke } from "@tauri-apps/api/core";

import { toRustOverlayConfig } from "../../entities/overlay";
import type { OverlayItem } from "../../entities/overlay";
import type { DisplayInfo } from "../types/display";

export function fetchDisplayInfo(): Promise<DisplayInfo | null> {
    return invoke<DisplayInfo | null>("get_display_info");
}

export async function previewOnDisplay(
    items: OverlayItem[],
    mediaBytes: ArrayBuffer,
    mediaType: string,
): Promise<void> {
    await invoke("set_overlay_config", {
        config: toRustOverlayConfig(items),
    });

    if (mediaType === "image/gif") {
        await invoke("render_gif", mediaBytes);
        return;
    }

    await invoke("render_image", mediaBytes);
}

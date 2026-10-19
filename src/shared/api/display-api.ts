import { invoke } from "@tauri-apps/api/core";

import { toRustOverlayConfig } from "../../entities/overlay";
import type { OverlayItem } from "../../entities/overlay";
import { hasPreviewMedia } from "../../features/display-editor/lib/preview-media";
import type { DisplayInfo } from "../types/display";

export function fetchDisplayInfo(): Promise<DisplayInfo | null> {
    return invoke<DisplayInfo | null>("get_display_info");
}

export async function previewOnDisplay(
    items: OverlayItem[],
    mediaBytes: ArrayBuffer,
    mediaType: string,
    mediaUrl: string | null = null,
): Promise<void> {
    if (!hasPreviewMedia(mediaUrl, mediaBytes, mediaType)) {
        throw new Error("Add media before previewing.");
    }

    await invoke("set_overlay_config", {
        config: toRustOverlayConfig(items),
    });

    if (mediaType === "image/gif") {
        await invoke("render_gif", mediaBytes);
        return;
    }

    await invoke("render_image", mediaBytes);
}

import { useState } from "react";

import type { OverlayItem } from "../../../entities/overlay";
import { previewOnDisplay } from "../../../shared/api/display-api";

export function useDisplayPreview(
    items: OverlayItem[],
    onStatus: (message: string) => void,
) {
    const [previewing, setPreviewing] = useState(false);

    async function sendPreview(mediaBytes: ArrayBuffer | null, mediaType: string | null) {
        if (!mediaBytes || !mediaType) {
            onStatus("Add media first.");
            return;
        }

        setPreviewing(true);
        onStatus("");

        try {
            await previewOnDisplay(items, mediaBytes, mediaType);
            onStatus("Sent to display.");
        } catch (error) {
            console.error("Preview failed:", error);
            onStatus("Preview failed.");
        } finally {
            setPreviewing(false);
        }
    }

    return { previewing, sendPreview };
}

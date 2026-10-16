import { OVERLAY_METRICS } from "./metrics";
import type { OverlayItem } from "./types";

export function createDefaultOverlayItems(): OverlayItem[] {
    return OVERLAY_METRICS.map((entry, index) => ({
        metric: entry.metric,
        enabled: true,
        x: 8,
        y: 8 + index * 32,
        size: 24,
        color: { r: 255, g: 255, b: 255, a: 255 },
    }));
}

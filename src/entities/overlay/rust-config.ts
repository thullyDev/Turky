import type { OverlayItem } from "./types";

export function toRustOverlayConfig(items: OverlayItem[]) {
    return {
        items: items
            .filter((item) => item.enabled)
            .map((item) => ({
                metric: item.metric,
                position: {
                    x: Math.round(item.x) || 0,
                    y: Math.round(item.y) || 0,
                },
                size: Number.isFinite(item.size) ? item.size : 0,
                color: item.color,
            })),
    };
}

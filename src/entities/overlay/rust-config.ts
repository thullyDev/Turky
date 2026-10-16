import { normalizeOverlayItem } from "./normalize-item";
import type { OverlayItem } from "./types";

export function toRustOverlayConfig(items: OverlayItem[]) {
    return {
        items: items
            .filter((item) => item.enabled)
            .map((item) => {
                const normalized = normalizeOverlayItem(item);

                return {
                    metric: normalized.metric,
                    position: {
                        x: Math.round(normalized.x) || 0,
                        y: Math.round(normalized.y) || 0,
                    },
                    label: normalized.label,
                    value: normalized.value,
                    widget: normalized.widget,
                };
            }),
    };
}

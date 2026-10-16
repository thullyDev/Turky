import { createDefaultLabelBlock, createDefaultValueBlock } from "./text-block";
import { OVERLAY_METRICS } from "./metrics";
import { createDefaultWidgetConfig } from "./widget-config";
import type { OverlayItem } from "./types";

export function createDefaultOverlayItems(): OverlayItem[] {
    return OVERLAY_METRICS.map((entry, index) => ({
        metric: entry.metric,
        enabled: true,
        x: 8,
        y: 8 + index * 32,
        label: createDefaultLabelBlock(),
        value: createDefaultValueBlock(),
        widget: createDefaultWidgetConfig(),
    }));
}

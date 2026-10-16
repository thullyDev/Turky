import { useCallback } from "react";

import type { OverlayItem, OverlayMetric } from "../../../entities/overlay";

export function useOverlayItemPatch(
    items: OverlayItem[],
    onChange: (items: OverlayItem[]) => void,
) {
    return useCallback(
        (metric: OverlayMetric, patch: Partial<OverlayItem>) => {
            onChange(items.map((item) => (item.metric === metric ? { ...item, ...patch } : item)));
        },
        [items, onChange],
    );
}

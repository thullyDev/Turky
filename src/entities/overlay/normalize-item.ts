import { legacyTextBlock, type TextBlockStyle } from "./text-block";
import { createDefaultTextStyle } from "./text-style";
import type { OverlayItem } from "./types";

function hasBlock(block: TextBlockStyle | undefined): block is TextBlockStyle {
    return Boolean(block && block.size > 0);
}

export function normalizeOverlayItem(item: OverlayItem): OverlayItem {
    const legacySize = item.size ?? 24;
    const legacyColor = item.color ?? { r: 255, g: 255, b: 255, a: 255 };
    const legacyStyle = item.style ?? createDefaultTextStyle();

    const label = hasBlock(item.label)
        ? item.label
        : legacyTextBlock(Math.max(10, legacySize * 0.46), legacyColor, legacyStyle);

    const value = hasBlock(item.value)
        ? item.value
        : legacyTextBlock(legacySize, legacyColor, legacyStyle);

    return {
        ...item,
        label,
        value,
        widget: item.widget,
    };
}

export function normalizeOverlayItems(items: OverlayItem[]): OverlayItem[] {
    return items.map(normalizeOverlayItem);
}

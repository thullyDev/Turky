import type { CSSProperties } from "react";

import { colorToCss } from "./color";
import type { OverlayItem } from "./types";

export const STAT_FONT_FAMILY = "Roboto, sans-serif";

export function statTextStyle(item: OverlayItem, scale: number): CSSProperties {
    return {
        fontFamily: STAT_FONT_FAMILY,
        fontSize: item.size * scale,
        fontWeight: 400,
        lineHeight: 1,
        letterSpacing: 0,
        color: colorToCss(item.color),
    };
}

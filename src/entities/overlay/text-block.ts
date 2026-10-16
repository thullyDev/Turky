import type { OverlayColor } from "./types";
import { createDefaultTextStyle, type OverlayTextStyle } from "./text-style";

export interface TextBlockStyle {
    size: number;
    color: OverlayColor;
    typography: OverlayTextStyle;
    css: string;
}

export function createDefaultLabelBlock(): TextBlockStyle {
    return {
        size: 11,
        color: { r: 255, g: 255, b: 255, a: 255 },
        typography: {
            ...createDefaultTextStyle(),
            fontWeight: 700,
            uppercase: true,
            letterSpacing: 0.08,
        },
        css: "",
    };
}

export function createDefaultValueBlock(): TextBlockStyle {
    return {
        size: 24,
        color: { r: 255, g: 255, b: 255, a: 255 },
        typography: {
            ...createDefaultTextStyle(),
            fontWeight: 700,
        },
        css: "",
    };
}

export function legacyTextBlock(
    size: number,
    color: OverlayColor,
    typography: OverlayTextStyle,
): TextBlockStyle {
    return {
        size,
        color,
        typography,
        css: "",
    };
}

import type { OverlayColor } from "./types";
import { createDefaultTextStyle, type OverlayTextStyle } from "./text-style";

export interface TextBlockGradient {
    enabled: boolean;
    endColor: OverlayColor;
    angle: number;
}

export interface TextBlockStyle {
    size: number;
    color: OverlayColor;
    typography: OverlayTextStyle;
    gradient: TextBlockGradient;
    css: string;
}

export function createDefaultTextGradient(): TextBlockGradient {
    return {
        enabled: false,
        endColor: { r: 236, g: 72, b: 153, a: 255 },
        angle: 180,
    };
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
        gradient: createDefaultTextGradient(),
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
        gradient: createDefaultTextGradient(),
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
        gradient: createDefaultTextGradient(),
        css: "",
    };
}

import type { OverlayColor } from "./types";

export function colorToHex(color: OverlayColor): string {
    const channel = (value: number) => value.toString(16).padStart(2, "0");

    return `#${channel(color.r)}${channel(color.g)}${channel(color.b)}`;
}

export function hexToColor(hex: string): OverlayColor {
    const value = hex.replace("#", "");

    return {
        r: Number.parseInt(value.slice(0, 2), 16),
        g: Number.parseInt(value.slice(2, 4), 16),
        b: Number.parseInt(value.slice(4, 6), 16),
        a: 255,
    };
}

export function colorToCss(color: OverlayColor): string {
    return `rgba(${color.r}, ${color.g}, ${color.b}, ${color.a / 255})`;
}

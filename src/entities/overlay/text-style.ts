export type OverlayFontPreset = "roboto" | "inter" | "mono" | "serif" | "compact";

export interface OverlayTextStyle {
    fontPreset: OverlayFontPreset;
    fontWeight: 400 | 500 | 600 | 700;
    italic: boolean;
    letterSpacing: number;
    uppercase: boolean;
    customFontId?: string;
}

export const FONT_PRESET_OPTIONS: { id: OverlayFontPreset; label: string }[] = [
    { id: "roboto", label: "Roboto" },
    { id: "inter", label: "Inter" },
    { id: "mono", label: "Monospace" },
    { id: "serif", label: "Serif" },
    { id: "compact", label: "Compact" },
];

export const FONT_WEIGHT_OPTIONS: OverlayTextStyle["fontWeight"][] = [400, 500, 600, 700];

export function createDefaultTextStyle(): OverlayTextStyle {
    return {
        fontPreset: "roboto",
        fontWeight: 400,
        italic: false,
        letterSpacing: 0,
        uppercase: false,
    };
}

export function fontFamilyForTypography(
    typography: OverlayTextStyle,
    customFamilies: Record<string, string>,
): string {
    if (typography.customFontId) {
        const family = customFamilies[typography.customFontId];

        if (family) {
            return `"${family}", sans-serif`;
        }
    }

    return fontFamilyForPreset(typography.fontPreset);
}

export function fontFamilyForPreset(preset: OverlayFontPreset): string {
    switch (preset) {
        case "roboto":
            return "Roboto, sans-serif";
        case "inter":
            return "Inter, system-ui, sans-serif";
        case "mono":
            return '"JetBrains Mono", ui-monospace, monospace';
        case "serif":
            return 'Georgia, "Times New Roman", serif';
        case "compact":
            return "Inter, system-ui, sans-serif";
    }
}

export function letterSpacingForPreset(
    preset: OverlayFontPreset,
    letterSpacing: number,
): number {
    const base = preset === "compact" ? -0.4 : 0;

    return base + letterSpacing;
}

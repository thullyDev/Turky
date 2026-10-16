import type { TextBlockStyle } from "./text-block";
import type { OverlayWidgetConfig } from "./widget-config";
import type { OverlayItem } from "./types";

export interface StylePresetPatch {
    label?: Partial<TextBlockStyle> & {
        typography?: Partial<TextBlockStyle["typography"]>;
    };
    value?: Partial<TextBlockStyle> & {
        typography?: Partial<TextBlockStyle["typography"]>;
    };
    widget?: Partial<OverlayWidgetConfig>;
}

export interface StylePreset {
    id: string;
    name: string;
    description: string;
    patch: StylePresetPatch;
}

export const STYLE_PRESETS: StylePreset[] = [
    {
        id: "minimal",
        name: "Minimal",
        description: "Flat white type, thin bar, no glow.",
        patch: {
            label: {
                size: 10,
                color: { r: 200, g: 200, b: 204, a: 255 },
                typography: {
                    fontPreset: "inter",
                    fontWeight: 500,
                    italic: false,
                    letterSpacing: 0.02,
                    uppercase: false,
                },
                css: "",
            },
            value: {
                size: 20,
                color: { r: 255, g: 255, b: 255, a: 255 },
                typography: {
                    fontPreset: "inter",
                    fontWeight: 600,
                    italic: false,
                    letterSpacing: 0,
                    uppercase: false,
                },
                css: "",
            },
            widget: {
                trackColor: { r: 255, g: 255, b: 255, a: 30 },
                fillColor: { r: 255, g: 255, b: 255, a: 220 },
                barBorderRadius: 4,
                barTrackCss: "",
                barFillCss: "",
                ringTrackCss: "",
                ringFillCss: "",
            },
        },
    },
    {
        id: "gaming-glow",
        name: "Gaming glow",
        description: "Bold caps label, bright value, red ring accent.",
        patch: {
            label: {
                size: 11,
                color: { r: 255, g: 255, b: 255, a: 255 },
                typography: {
                    fontPreset: "roboto",
                    fontWeight: 700,
                    italic: false,
                    letterSpacing: 0.1,
                    uppercase: true,
                },
                css: "text-shadow: 0 0 6px rgba(0,0,0,0.85), 0 1px 2px rgba(0,0,0,0.9);",
            },
            value: {
                size: 26,
                color: { r: 255, g: 255, b: 255, a: 255 },
                typography: {
                    fontPreset: "roboto",
                    fontWeight: 700,
                    italic: false,
                    letterSpacing: 0,
                    uppercase: false,
                },
                css: "text-shadow: 0 0 8px rgba(0,0,0,0.9), 0 2px 4px rgba(0,0,0,0.85);",
            },
            widget: {
                kind: "ring",
                trackColor: { r: 40, g: 40, b: 44, a: 200 },
                fillColor: { r: 239, g: 68, b: 68, a: 255 },
                ringSize: 72,
                ringThickness: 8,
                ringTrackCss: "opacity: 0.85;",
                ringFillCss: "filter: drop-shadow(0 0 4px rgba(239,68,68,0.75));",
                barTrackCss: "background: linear-gradient(90deg, #1a1a1d, #2a2a2f);",
                barFillCss: "box-shadow: 0 0 8px rgba(239,68,68,0.6);",
                barBorderRadius: 999,
            },
        },
    },
    {
        id: "neon-bar",
        name: "Neon bar",
        description: "Stack layout with violet neon fill.",
        patch: {
            label: {
                size: 10,
                color: { r: 167, g: 139, b: 250, a: 255 },
                typography: {
                    fontPreset: "mono",
                    fontWeight: 600,
                    italic: false,
                    letterSpacing: 0.06,
                    uppercase: true,
                },
                css: "",
            },
            value: {
                size: 22,
                color: { r: 255, g: 255, b: 255, a: 255 },
                typography: {
                    fontPreset: "mono",
                    fontWeight: 600,
                    italic: false,
                    letterSpacing: 0,
                    uppercase: false,
                },
                css: "",
            },
            widget: {
                kind: "stack",
                trackColor: { r: 139, g: 92, b: 246, a: 35 },
                fillColor: { r: 139, g: 92, b: 246, a: 255 },
                barHeight: 8,
                barBorderRadius: 999,
                barTrackCss: "box-shadow: inset 0 0 6px rgba(0,0,0,0.5);",
                barFillCss:
                    "background: linear-gradient(90deg, #7c3aed, #a78bfa); box-shadow: 0 0 10px rgba(139,92,246,0.65);",
            },
        },
    },
    {
        id: "compact-mono",
        name: "Compact mono",
        description: "Dense monospace readout for small displays.",
        patch: {
            label: {
                size: 9,
                color: { r: 160, g: 160, b: 170, a: 255 },
                typography: {
                    fontPreset: "compact",
                    fontWeight: 500,
                    italic: false,
                    letterSpacing: -0.2,
                    uppercase: false,
                },
                css: "",
            },
            value: {
                size: 16,
                color: { r: 240, g: 240, b: 245, a: 255 },
                typography: {
                    fontPreset: "mono",
                    fontWeight: 500,
                    italic: false,
                    letterSpacing: 0,
                    uppercase: false,
                },
                css: "",
            },
            widget: {
                kind: "text",
                barBorderRadius: 2,
            },
        },
    },
];

export function applyStylePreset(item: OverlayItem, presetId: string): OverlayItem {
    const preset = STYLE_PRESETS.find((entry) => entry.id === presetId);

    if (!preset) {
        return item;
    }

    const patch = preset.patch;

    return {
        ...item,
        label: patch.label
            ? {
                  ...item.label,
                  ...patch.label,
                  typography: patch.label.typography
                      ? { ...item.label.typography, ...patch.label.typography }
                      : item.label.typography,
              }
            : item.label,
        value: patch.value
            ? {
                  ...item.value,
                  ...patch.value,
                  typography: patch.value.typography
                      ? { ...item.value.typography, ...patch.value.typography }
                      : item.value.typography,
              }
            : item.value,
        widget: patch.widget
            ? {
                  ...item.widget,
                  ...patch.widget,
              }
            : item.widget,
    };
}

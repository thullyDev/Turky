import type { OverlayColor } from "./types";

export type WidgetKind = "text" | "bar" | "ring" | "stack";

export interface OverlayWidgetConfig {
    kind: WidgetKind;
    label: string;
    showLabel: boolean;
    showValue: boolean;
    barWidth: number;
    barHeight: number;
    barBorderRadius: number;
    barTrackCss: string;
    barFillCss: string;
    ringSize: number;
    ringThickness: number;
    ringTrackCss: string;
    ringFillCss: string;
    trackColor: OverlayColor;
    fillColor: OverlayColor;
}

export const WIDGET_KIND_OPTIONS: { id: WidgetKind; label: string }[] = [
    { id: "text", label: "Text" },
    { id: "bar", label: "Bar" },
    { id: "ring", label: "Ring" },
    { id: "stack", label: "Stack" },
];

export function createDefaultWidgetConfig(): OverlayWidgetConfig {
    return {
        kind: "text",
        label: "",
        showLabel: true,
        showValue: true,
        barWidth: 128,
        barHeight: 10,
        barBorderRadius: 999,
        barTrackCss: "",
        barFillCss: "",
        ringSize: 56,
        ringThickness: 7,
        ringTrackCss: "",
        ringFillCss: "",
        trackColor: { r: 255, g: 255, b: 255, a: 40 },
        fillColor: { r: 139, g: 92, b: 246, a: 255 },
    };
}

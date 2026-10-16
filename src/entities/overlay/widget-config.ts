import type { OverlayColor } from "./types";
import type { BarChromeStyle, RingChromeStyle, RingLineCap } from "./widget-chrome";

export type WidgetKind = "text" | "bar" | "ring" | "stack";

export interface OverlayWidgetConfig {
    kind: WidgetKind;
    label: string;
    showLabel: boolean;
    showValue: boolean;
    barWidth: number;
    barHeight: number;
    barBorderRadius: number;
    barStyle: BarChromeStyle;
    barDotSize: number;
    barDotGap: number;
    barSegmentSize: number;
    barSegmentGap: number;
    barTrackCss: string;
    barFillCss: string;
    ringSize: number;
    ringThickness: number;
    ringStyle: RingChromeStyle;
    ringLineCap: RingLineCap;
    ringSegmentCount: number;
    ringSegmentGap: number;
    ringDashLength: number;
    ringDashGap: number;
    ringDotSize: number;
    ringDotGap: number;
    ringTrackCss: string;
    ringFillCss: string;
    chromeCss: string;
    trackColor: OverlayColor;
    fillColor: OverlayColor;
    fillGradientColor: OverlayColor;
    fillGradientAngle: number;
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
        barStyle: "solid",
        barDotSize: 3,
        barDotGap: 8,
        barSegmentSize: 10,
        barSegmentGap: 4,
        barTrackCss: "",
        barFillCss: "",
        ringSize: 72,
        ringThickness: 8,
        ringStyle: "gradient",
        ringLineCap: "round",
        ringSegmentCount: 12,
        ringSegmentGap: 4,
        ringDashLength: 10,
        ringDashGap: 6,
        ringDotSize: 3,
        ringDotGap: 5,
        ringTrackCss: "",
        ringFillCss: "",
        chromeCss: "",
        trackColor: { r: 186, g: 230, b: 253, a: 90 },
        fillColor: { r: 168, g: 85, b: 247, a: 255 },
        fillGradientColor: { r: 34, g: 211, b: 238, a: 255 },
        fillGradientAngle: 180,
    };
}

/** Sensible chrome defaults when the user picks bar / ring / stack in the inspector. */
export function widgetPatchForKind(kind: WidgetKind): Partial<OverlayWidgetConfig> {
    if (kind === "ring") {
        return {
            kind: "ring",
            ringStyle: "gradient",
            ringSize: 72,
            ringThickness: 8,
            ringLineCap: "round",
            trackColor: { r: 186, g: 230, b: 253, a: 90 },
            fillColor: { r: 168, g: 85, b: 247, a: 255 },
            fillGradientColor: { r: 34, g: 211, b: 238, a: 255 },
            fillGradientAngle: 180,
        };
    }

    if (kind === "bar") {
        return { kind: "bar", barStyle: "solid" };
    }

    if (kind === "stack") {
        return { kind: "stack", barStyle: "solid" };
    }

    return { kind: "text" };
}

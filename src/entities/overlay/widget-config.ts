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
        ringSize: 56,
        ringThickness: 7,
        ringStyle: "solid",
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
        trackColor: { r: 255, g: 255, b: 255, a: 40 },
        fillColor: { r: 139, g: 92, b: 246, a: 255 },
        fillGradientColor: { r: 56, g: 189, b: 248, a: 255 },
        fillGradientAngle: 90,
    };
}

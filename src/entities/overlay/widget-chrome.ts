import type { CSSProperties } from "react";

import { colorToCss } from "./color";
import type { OverlayWidgetConfig } from "./widget-config";

export type BarChromeStyle = "solid" | "gradient" | "dotted" | "segmented";
export type RingChromeStyle = "solid" | "gradient" | "dashed" | "dotted" | "segmented";
export type RingLineCap = "round" | "butt" | "square";

export const BAR_CHROME_STYLE_OPTIONS: { id: BarChromeStyle; label: string }[] = [
    { id: "solid", label: "Solid" },
    { id: "gradient", label: "Gradient" },
    { id: "dotted", label: "Dotted" },
    { id: "segmented", label: "Segmented" },
];

export const RING_CHROME_STYLE_OPTIONS: { id: RingChromeStyle; label: string }[] = [
    { id: "solid", label: "Solid arc" },
    { id: "gradient", label: "Gradient arc" },
    { id: "dashed", label: "Dashed" },
    { id: "dotted", label: "Dotted" },
    { id: "segmented", label: "Segment blocks" },
];

export const RING_LINE_CAP_OPTIONS: { id: RingLineCap; label: string }[] = [
    { id: "round", label: "Round" },
    { id: "butt", label: "Flat" },
    { id: "square", label: "Square" },
];

export const PROGRESS_KIND_HELP: Record<"bar" | "ring" | "stack", string> = {
    bar: "A horizontal progress strip. Good when space is tight; label and value are optional.",
    ring: "Circular gauge around the center. Label and value sit inside the ring.",
    stack:
        "HUD column: label on top, bar in the middle, value below — all stacked vertically.",
};

function gradientEnd(widget: OverlayWidgetConfig): string {
    if (widget.fillGradientColor.a > 0) {
        return colorToCss(widget.fillGradientColor);
    }

    return colorToCss(widget.fillColor);
}

export function barTrackStyle(
    widget: OverlayWidgetConfig,
    track: string,
    scale: number,
): CSSProperties {
    const radius = widget.barBorderRadius * scale;
    const base: CSSProperties = {
        background: track,
        borderRadius: radius,
    };

    if (widget.barStyle === "dotted") {
        const dot = Math.max(2, widget.barDotSize * scale);
        const gap = Math.max(4, widget.barDotGap * scale);

        return {
            ...base,
            background: "transparent",
            backgroundImage: `radial-gradient(circle, ${track} ${dot}px, transparent ${dot}px)`,
            backgroundSize: `${gap}px 100%`,
            backgroundRepeat: "repeat-x",
        };
    }

    if (widget.barStyle === "segmented") {
        const seg = Math.max(4, widget.barSegmentSize * scale);
        const gap = Math.max(2, widget.barSegmentGap * scale);

        return {
            ...base,
            background: `repeating-linear-gradient(90deg, ${track} 0, ${track} ${seg}px, transparent ${seg}px, transparent ${seg + gap}px)`,
        };
    }

    return base;
}

export function barFillStyle(
    widget: OverlayWidgetConfig,
    fill: string,
    scale: number,
): CSSProperties {
    const radius = widget.barBorderRadius * scale;
    const end = gradientEnd(widget);

    if (widget.barStyle === "gradient") {
        return {
            background: `linear-gradient(${widget.fillGradientAngle}deg, ${fill}, ${end})`,
            borderRadius: radius,
        };
    }

    if (widget.barStyle === "dotted") {
        const dot = Math.max(2, widget.barDotSize * scale);
        const gap = Math.max(4, widget.barDotGap * scale);

        return {
            background: "transparent",
            backgroundImage: `radial-gradient(circle, ${fill} ${dot}px, transparent ${dot}px)`,
            backgroundSize: `${gap}px 100%`,
            backgroundRepeat: "repeat-x",
            borderRadius: radius,
        };
    }

    if (widget.barStyle === "segmented") {
        const seg = Math.max(4, widget.barSegmentSize * scale);
        const gap = Math.max(2, widget.barSegmentGap * scale);

        return {
            background: `repeating-linear-gradient(90deg, ${fill} 0, ${fill} ${seg}px, transparent ${seg}px, transparent ${seg + gap}px)`,
            borderRadius: radius,
        };
    }

    return {
        background: fill,
        borderRadius: radius,
    };
}

export function ringDashArray(
    widget: OverlayWidgetConfig,
    circumference: number,
    ratio: number,
    forTrack: boolean,
): string {
    const style = widget.ringStyle;

    if (style === "dashed") {
        const dash = Math.max(4, widget.ringDashLength);
        const gap = Math.max(2, widget.ringDashGap);

        if (forTrack) {
            return `${dash} ${gap}`;
        }

        return `${dash * ratio * (circumference / (dash + gap))} ${circumference}`;
    }

    if (style === "dotted") {
        const dot = Math.max(2, widget.ringDotSize);
        const gap = Math.max(2, widget.ringDotGap);

        if (forTrack) {
            return `${dot} ${gap}`;
        }

        return `${Math.max(0.001, circumference * ratio)} ${circumference}`;
    }

    if (style === "segmented") {
        const count = Math.max(4, widget.ringSegmentCount);
        const segLen = circumference / count;
        const gap = Math.max(1, widget.ringSegmentGap);
        const block = Math.max(1, segLen - gap);

        if (forTrack) {
            return `${block} ${gap}`.repeat(count).trim();
        }

        const activeSegments = Math.max(0, Math.round(count * ratio));
        const parts: string[] = [];

        for (let index = 0; index < count; index += 1) {
            if (index < activeSegments) {
                parts.push(`${block}`, `${gap}`);
            } else {
                parts.push("0", `${block + gap}`);
            }
        }

        return parts.join(" ");
    }

    if (forTrack) {
        return `${circumference}`;
    }

    return `${circumference * ratio} ${circumference}`;
}

export function ringUsesGradient(widget: OverlayWidgetConfig): boolean {
    return widget.ringStyle === "gradient";
}

export function svgGradientLine(size: number, angleDeg: number) {
    const center = size / 2;
    const radians = ((angleDeg - 90) * Math.PI) / 180;
    const extent = size / 2;

    return {
        x1: center + Math.cos(radians) * extent,
        y1: center + Math.sin(radians) * extent,
        x2: center - Math.cos(radians) * extent,
        y2: center - Math.sin(radians) * extent,
    };
}

export function ringStrokeColor(
    widget: OverlayWidgetConfig,
    fill: string,
    gradientId: string | undefined,
): string {
    if (widget.ringStyle === "gradient" && gradientId) {
        return `url(#${gradientId})`;
    }

    return fill;
}

export type OverlayMetric =
    | "cpu"
    | "ram"
    | "gpu"
    | "gpuMemory"
    | "cpuTemp"
    | "gpuTemp"
    | "text";

export type TextAnchor =
    | "topLeft"
    | "topRight"
    | "bottomLeft"
    | "bottomRight"
    | "center";

export interface OverlayColor {
    r: number;
    g: number;
    b: number;
    a: number;
}

export interface OverlayItem {
    metric: OverlayMetric;
    enabled: boolean;
    x: number;
    y: number;
    anchor: TextAnchor;
    font: string;
    size: number;
    color: OverlayColor;
    text: string;
}

export const overlayMetrics: { metric: OverlayMetric; label: string }[] = [
    { metric: "cpu", label: "CPU" },
    { metric: "ram", label: "RAM" },
    { metric: "gpu", label: "GPU" },
    { metric: "gpuMemory", label: "GPU memory" },
    { metric: "cpuTemp", label: "CPU temp" },
    { metric: "gpuTemp", label: "GPU temp" },
    { metric: "text", label: "Text" },
];

export const defaultFont = "Roboto";

export const textAnchors: { value: TextAnchor; label: string }[] = [
    { value: "topLeft", label: "Top left" },
    { value: "topRight", label: "Top right" },
    { value: "bottomLeft", label: "Bottom left" },
    { value: "bottomRight", label: "Bottom right" },
    { value: "center", label: "Center" },
];

export function defaultOverlayItems(): OverlayItem[] {
    return overlayMetrics.map((entry, index) => ({
        metric: entry.metric,
        enabled: false,
        x: 24,
        y: 24 + index * 48,
        anchor: "topLeft",
        font: defaultFont,
        size: 32,
        color: { r: 255, g: 255, b: 255, a: 255 },
        text: entry.metric === "text" ? "Turky" : "",
    }));
}

export function toOverlayConfig(items: OverlayItem[]) {
    return {
        items: items
            .filter((item) => item.enabled)
            .map((item) => ({
                metric: item.metric,
                position: {
                    x: Math.round(item.x) || 0,
                    y: Math.round(item.y) || 0,
                    anchor: item.anchor,
                },
                font: item.font || defaultFont,
                size: Number.isFinite(item.size) ? item.size : 0,
                color: item.color,
                text: item.text,
            })),
    };
}

export function colorToHex(color: OverlayColor): string {
    const channel = (value: number) =>
        value.toString(16).padStart(2, "0");

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

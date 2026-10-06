export type OverlayMetric =
    | "cpu"
    | "ram"
    | "gpu"
    | "gpuMemory"
    | "cpuTemp"
    | "gpuTemp";

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
    size: number;
    color: OverlayColor;
}

export const overlayMetrics: { metric: OverlayMetric; label: string }[] = [
    { metric: "cpu", label: "CPU" },
    { metric: "ram", label: "RAM" },
    { metric: "gpu", label: "GPU" },
    { metric: "gpuMemory", label: "GPU memory" },
    { metric: "cpuTemp", label: "CPU temp" },
    { metric: "gpuTemp", label: "GPU temp" },
];

export function defaultOverlayItems(): OverlayItem[] {
    return overlayMetrics.map((entry, index) => ({
        metric: entry.metric,
        enabled: true,
        x: 8,
        y: 8 + index * 32,
        size: 24,
        color: { r: 255, g: 255, b: 255, a: 255 },
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
                },
                size: Number.isFinite(item.size) ? item.size : 0,
                color: item.color,
            })),
    };
}

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

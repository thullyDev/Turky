export type OverlayMetric =
    | "cpu"
    | "ram"
    | "gpu"
    | "gpuMemory"
    | "cpuTemp"
    | "gpuTemp"
    | "fps";

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

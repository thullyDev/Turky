import type { TextBlockStyle } from "./text-block";
import type { OverlayTextStyle } from "./text-style";
import type { OverlayWidgetConfig } from "./widget-config";

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
    label: TextBlockStyle;
    value: TextBlockStyle;
    widget: OverlayWidgetConfig;
    size?: number;
    color?: OverlayColor;
    style?: OverlayTextStyle;
}

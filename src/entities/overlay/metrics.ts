import type { OverlayMetric } from "./types";

export const OVERLAY_METRICS: { metric: OverlayMetric; label: string }[] = [
    { metric: "cpu", label: "CPU" },
    { metric: "ram", label: "RAM" },
    { metric: "gpu", label: "GPU" },
    { metric: "gpuMemory", label: "GPU memory" },
    { metric: "cpuTemp", label: "CPU temp" },
    { metric: "gpuTemp", label: "GPU temp" },
    { metric: "fps", label: "FPS" },
];

export function metricLabel(metric: OverlayMetric): string {
    return OVERLAY_METRICS.find((entry) => entry.metric === metric)?.label ?? metric;
}

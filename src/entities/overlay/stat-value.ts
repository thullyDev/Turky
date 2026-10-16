import { metricLabel } from "./metrics";
import type { OverlayItem, OverlayMetric } from "./types";

export interface StatSample {
    ratio: number | null;
    valueText: string;
}

const SAMPLES: Record<OverlayMetric, StatSample> = {
    cpu: { ratio: 0.42, valueText: "42%" },
    ram: { ratio: 0.5, valueText: "8.0/16.0 GB" },
    gpu: { ratio: 0.18, valueText: "18%" },
    gpuMemory: { ratio: 0.25, valueText: "2.0/8.0 GB" },
    cpuTemp: { ratio: 0.52, valueText: "52°C" },
    gpuTemp: { ratio: 0.61, valueText: "61°C" },
    fps: { ratio: 0.75, valueText: "60" },
};

export function sampleForMetric(metric: OverlayMetric): StatSample {
    return SAMPLES[metric];
}

export function resolveWidgetLabel(item: OverlayItem): string {
    const trimmed = item.widget.label.trim();

    if (trimmed) {
        return trimmed;
    }

    return metricLabel(item.metric);
}

export function sampleStatLabel(metric: OverlayMetric): string {
    const sample = sampleForMetric(metric);
    const name = metricLabel(metric);

    return `${name} ${sample.valueText}`;
}

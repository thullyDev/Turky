import type { OverlayMetric } from "./types";

export function sampleStatLabel(metric: OverlayMetric): string {
    switch (metric) {
        case "cpu":
            return "CPU 42%";
        case "ram":
            return "RAM 8.0/16.0 GB";
        case "gpu":
            return "GPU 18%";
        case "gpuMemory":
            return "VRAM 2.0/8.0 GB";
        case "cpuTemp":
            return "CPU 52°C";
        case "gpuTemp":
            return "GPU 61°C";
        case "fps":
            return "FPS 60";
    }
}

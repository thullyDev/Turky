import type { OverlayItem } from "../../../../entities/overlay";

export function scopedWidgetCss(metric: string, item: OverlayItem): string {
    const root = `.stat-scope-${metric}`;
    const rules: string[] = [];

    if (item.label.css.trim()) {
        rules.push(`${root} .stat-widget-label { ${item.label.css.trim()} }`);
    }

    if (item.value.css.trim()) {
        rules.push(`${root} .stat-widget-value { ${item.value.css.trim()} }`);
    }

    if (item.widget.barTrackCss.trim()) {
        rules.push(`${root} .stat-widget-bar-track { ${item.widget.barTrackCss.trim()} }`);
    }

    if (item.widget.barFillCss.trim()) {
        rules.push(`${root} .stat-widget-bar-fill { ${item.widget.barFillCss.trim()} }`);
    }

    if (item.widget.ringTrackCss.trim()) {
        rules.push(`${root} .stat-widget-ring-track { ${item.widget.ringTrackCss.trim()} }`);
    }

    if (item.widget.ringFillCss.trim()) {
        rules.push(`${root} .stat-widget-ring-fill { ${item.widget.ringFillCss.trim()} }`);
    }

    return rules.join("\n");
}

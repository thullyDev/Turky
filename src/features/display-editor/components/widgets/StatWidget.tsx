import {
    colorToCss,
    resolveWidgetLabel,
    sampleForMetric,
    textBlockStyle,
    type OverlayItem,
} from "../../../../entities/overlay";

import { scopedWidgetCss } from "./stat-widget-scope";

interface StatWidgetProps {
    item: OverlayItem;
    scale: number;
    customFamilies: Record<string, string>;
}

function fillColor(item: OverlayItem): string {
    if (item.widget.fillColor.a > 0) {
        return colorToCss(item.widget.fillColor);
    }

    return colorToCss(item.value.color);
}

export function StatWidget({ item, scale, customFamilies }: StatWidgetProps) {
    const sample = sampleForMetric(item.metric);
    const ratio = sample.ratio ?? 0;
    const label = resolveWidgetLabel(item);
    const labelStyle = textBlockStyle(item.label, scale, customFamilies);
    const valueStyle = textBlockStyle(item.value, scale, customFamilies);
    const track = colorToCss(item.widget.trackColor);
    const fill = fillColor(item);
    const scopeClass = `stat-scope-${item.metric}`;
    const scopedCss = scopedWidgetCss(item.metric, item);
    const barRadius = item.widget.barBorderRadius * scale;

    if (item.widget.kind === "text") {
        return (
            <div className={`stat-widget ${scopeClass}`}>
                {scopedCss ? <style>{scopedCss}</style> : null}
                {item.widget.showLabel ? (
                    <span className="stat-widget-label" style={labelStyle}>
                        {label}
                    </span>
                ) : null}
                {item.widget.showValue ? (
                    <span className="stat-widget-value" style={valueStyle}>
                        {sample.valueText}
                    </span>
                ) : null}
            </div>
        );
    }

    if (item.widget.kind === "bar") {
        const width = item.widget.barWidth * scale;
        const height = item.widget.barHeight * scale;

        return (
            <div className={`stat-widget stat-widget-bar ${scopeClass}`} style={{ width }}>
                {scopedCss ? <style>{scopedCss}</style> : null}
                {item.widget.showLabel ? (
                    <span className="stat-widget-label" style={labelStyle}>
                        {label}
                    </span>
                ) : null}
                <div
                    className="stat-widget-bar-track"
                    style={{ height, background: track, borderRadius: barRadius }}
                >
                    <div
                        className="stat-widget-bar-fill"
                        style={{
                            width: `${Math.round(ratio * 100)}%`,
                            background: fill,
                            borderRadius: barRadius,
                        }}
                    />
                </div>
                {item.widget.showValue ? (
                    <span className="stat-widget-value" style={valueStyle}>
                        {sample.valueText}
                    </span>
                ) : null}
            </div>
        );
    }

    if (item.widget.kind === "ring") {
        const size = item.widget.ringSize * scale;
        const stroke = item.widget.ringThickness * scale;
        const radius = (size - stroke) / 2;
        const circumference = 2 * Math.PI * radius;
        const dash = circumference * ratio;

        return (
            <div className={`stat-widget stat-widget-ring ${scopeClass}`} style={{ width: size }}>
                {scopedCss ? <style>{scopedCss}</style> : null}
                <svg width={size} height={size} viewBox={`0 0 ${size} ${size}`}>
                    <circle
                        className="stat-widget-ring-track"
                        cx={size / 2}
                        cy={size / 2}
                        r={radius}
                        fill="none"
                        stroke={track}
                        strokeWidth={stroke}
                    />
                    <circle
                        className="stat-widget-ring-fill"
                        cx={size / 2}
                        cy={size / 2}
                        r={radius}
                        fill="none"
                        stroke={fill}
                        strokeWidth={stroke}
                        strokeLinecap="round"
                        strokeDasharray={`${dash} ${circumference}`}
                        transform={`rotate(-90 ${size / 2} ${size / 2})`}
                    />
                </svg>
                <div className="stat-widget-ring-center">
                    {item.widget.showLabel ? (
                        <span className="stat-widget-label" style={labelStyle}>
                            {label}
                        </span>
                    ) : null}
                    {item.widget.showValue ? (
                        <span className="stat-widget-value" style={valueStyle}>
                            {sample.valueText}
                        </span>
                    ) : null}
                </div>
            </div>
        );
    }

    const width = item.widget.barWidth * scale;
    const height = item.widget.barHeight * scale;

    return (
        <div className={`stat-widget stat-widget-stack ${scopeClass}`} style={{ width }}>
            {scopedCss ? <style>{scopedCss}</style> : null}
            {item.widget.showLabel ? (
                <span className="stat-widget-label" style={labelStyle}>
                    {label}
                </span>
            ) : null}
            <div
                className="stat-widget-bar-track"
                style={{ height, background: track, borderRadius: barRadius }}
            >
                <div
                    className="stat-widget-bar-fill"
                    style={{
                        width: `${Math.round(ratio * 100)}%`,
                        background: fill,
                        borderRadius: barRadius,
                    }}
                />
            </div>
            {item.widget.showValue ? (
                <span className="stat-widget-value" style={valueStyle}>
                    {sample.valueText}
                </span>
            ) : null}
        </div>
    );
}

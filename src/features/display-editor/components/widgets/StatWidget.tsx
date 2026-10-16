import type { CSSProperties, ReactNode } from "react";

import {
    resolveWidgetLabel,
    sampleForMetric,
    textBlockStyle,
    colorToCss,
    type OverlayItem,
} from "../../../../entities/overlay";

import { scopedWidgetCss } from "./stat-widget-scope";
import { BarChrome } from "./BarChrome";
import { RingChrome } from "./RingChrome";

export type StatInspectPart = "label" | "value" | "chrome";

export interface StatWidgetInspect {
    active: StatInspectPart | null;
    onSelect: (part: StatInspectPart) => void;
}

interface StatWidgetProps {
    item: OverlayItem;
    scale: number;
    customFamilies: Record<string, string>;
    inspect?: StatWidgetInspect;
}

function fillColor(item: OverlayItem): string {
    if (item.widget.fillColor.a > 0) {
        return colorToCss(item.widget.fillColor);
    }

    return colorToCss(item.value.color);
}

function InspectTarget({
    part,
    inspect,
    className,
    style,
    children,
}: {
    part: StatInspectPart;
    inspect: StatWidgetInspect | undefined;
    className?: string;
    style?: CSSProperties;
    children: ReactNode;
}) {
    if (!inspect) {
        return (
            <span className={className} style={style}>
                {children}
            </span>
        );
    }

    const active = inspect.active === part;

    return (
        <button
            type="button"
            className={`stat-widget-hit ${className ?? ""}${active ? " is-inspect-active" : ""}`}
            style={style}
            onClick={(event) => {
                event.stopPropagation();
                inspect.onSelect(part);
            }}
        >
            {children}
        </button>
    );
}

export function StatWidget({ item, scale, customFamilies, inspect }: StatWidgetProps) {
    const sample = sampleForMetric(item.metric);
    const ratio = sample.ratio ?? 0;
    const label = resolveWidgetLabel(item);
    const labelStyle = textBlockStyle(item.label, scale, customFamilies);
    const valueStyle = textBlockStyle(item.value, scale, customFamilies);
    const track = colorToCss(item.widget.trackColor);
    const fill = fillColor(item);
    const scopeClass = `stat-scope-${item.metric}`;
    const scopedCss = scopedWidgetCss(item.metric, item);
    const rootClass = inspect ? "stat-widget stat-widget-is-inspect" : "stat-widget";
    const chromeShellClass = item.widget.chromeCss.trim()
        ? "stat-widget-chrome-shell has-custom-chrome"
        : "stat-widget-chrome-shell";

    const labelNode = item.widget.showLabel ? (
        <InspectTarget part="label" inspect={inspect} className="stat-widget-label" style={labelStyle}>
            {label}
        </InspectTarget>
    ) : null;

    const valueNode = item.widget.showValue ? (
        <InspectTarget part="value" inspect={inspect} className="stat-widget-value" style={valueStyle}>
            {sample.valueText}
        </InspectTarget>
    ) : null;

    const chromePlaceholder =
        inspect && item.widget.kind === "text" ? (
            <InspectTarget part="chrome" inspect={inspect} className="stat-widget-chrome-slot">
                <span className="stat-widget-chrome-slot-label">Bar / ring / stack</span>
            </InspectTarget>
        ) : null;

    if (item.widget.kind === "text") {
        return (
            <div className={`${rootClass} ${scopeClass}`}>
                {scopedCss ? <style>{scopedCss}</style> : null}
                {labelNode}
                {valueNode}
                {chromePlaceholder}
            </div>
        );
    }

    if (item.widget.kind === "bar") {
        const width = item.widget.barWidth * scale;

        return (
            <div className={`${rootClass} stat-widget-bar ${scopeClass}`} style={{ width }}>
                {scopedCss ? <style>{scopedCss}</style> : null}
                {labelNode}
                <InspectTarget part="chrome" inspect={inspect} className="stat-widget-chrome-bar">
                    <div className={chromeShellClass}>
                        <BarChrome widget={item.widget} track={track} fill={fill} ratio={ratio} scale={scale} />
                    </div>
                </InspectTarget>
                {valueNode}
            </div>
        );
    }

    if (item.widget.kind === "ring") {
        const size = item.widget.ringSize * scale;
        const innerPad = item.widget.ringThickness * scale * 0.35;

        return (
            <div
                className={`${rootClass} stat-widget-ring ${scopeClass}`}
                style={{ width: size, height: size }}
            >
                {scopedCss ? <style>{scopedCss}</style> : null}
                <InspectTarget part="chrome" inspect={inspect} className="stat-widget-chrome-ring">
                    <div className={chromeShellClass}>
                        <RingChrome
                            widget={item.widget}
                            metric={item.metric}
                            track={track}
                            fill={fill}
                            ratio={ratio}
                            scale={scale}
                        />
                    </div>
                </InspectTarget>
                <div className="stat-widget-ring-center" style={{ padding: innerPad }}>
                    {labelNode}
                    {valueNode}
                </div>
            </div>
        );
    }

    const width = item.widget.barWidth * scale;

    return (
        <div className={`${rootClass} stat-widget-stack ${scopeClass}`} style={{ width }}>
            {scopedCss ? <style>{scopedCss}</style> : null}
            {labelNode}
            <InspectTarget part="chrome" inspect={inspect} className="stat-widget-chrome-bar">
                <div className={chromeShellClass}>
                    <BarChrome widget={item.widget} track={track} fill={fill} ratio={ratio} scale={scale} />
                </div>
            </InspectTarget>
            {valueNode}
        </div>
    );
}

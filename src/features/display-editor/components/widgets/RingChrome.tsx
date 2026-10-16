import {
    colorToCss,
    ringDashArray,
    ringStrokeColor,
    ringUsesGradient,
    svgGradientLine,
    type OverlayWidgetConfig,
} from "../../../../entities/overlay";

interface RingChromeProps {
    widget: OverlayWidgetConfig;
    metric: string;
    track: string;
    fill: string;
    ratio: number;
    scale: number;
}

export function RingChrome({ widget, metric, track, fill, ratio, scale }: RingChromeProps) {
    const size = widget.ringSize * scale;
    const stroke = widget.ringThickness * scale;
    const radius = (size - stroke) / 2;
    const circumference = 2 * Math.PI * radius;
    const gradientId = `ring-grad-${metric}`;
    const useGradient = ringUsesGradient(widget);
    const trackDash = ringDashArray(widget, circumference, 1, true);
    const fillDash = ringDashArray(widget, circumference, ratio, false);
    const fillStroke = ringStrokeColor(widget, fill, gradientId);
    const gradientLine = svgGradientLine(size, widget.fillGradientAngle);

    return (
        <svg width={size} height={size} viewBox={`0 0 ${size} ${size}`} className="stat-widget-ring-svg">
            {useGradient ? (
                <defs>
                    <linearGradient
                        id={gradientId}
                        gradientUnits="userSpaceOnUse"
                        x1={gradientLine.x1}
                        y1={gradientLine.y1}
                        x2={gradientLine.x2}
                        y2={gradientLine.y2}
                    >
                        <stop offset="0%" stopColor={fill} />
                        <stop offset="100%" stopColor={colorToCss(widget.fillGradientColor)} />
                    </linearGradient>
                </defs>
            ) : null}
            <circle
                className="stat-widget-ring-track"
                cx={size / 2}
                cy={size / 2}
                r={radius}
                fill="none"
                stroke={track}
                strokeWidth={stroke}
                strokeLinecap={widget.ringLineCap}
                strokeDasharray={
                    widget.ringStyle === "solid" || widget.ringStyle === "gradient"
                        ? undefined
                        : trackDash
                }
            />
            <circle
                className="stat-widget-ring-fill"
                cx={size / 2}
                cy={size / 2}
                r={radius}
                fill="none"
                stroke={useGradient ? fillStroke : fill}
                strokeWidth={stroke}
                strokeLinecap={widget.ringLineCap}
                strokeDasharray={
                    widget.ringStyle === "solid" || widget.ringStyle === "gradient"
                        ? `${circumference * ratio} ${circumference}`
                        : fillDash
                }
                transform={`rotate(-90 ${size / 2} ${size / 2})`}
            />
        </svg>
    );
}

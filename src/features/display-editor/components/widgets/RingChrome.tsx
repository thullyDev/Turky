import {
    colorToCss,
    ringDashArray,
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

function isArcRingStyle(widget: OverlayWidgetConfig): boolean {
    return widget.ringStyle === "solid" || widget.ringStyle === "gradient";
}

export function RingChrome({ widget, metric, track, fill, ratio, scale }: RingChromeProps) {
    const size = widget.ringSize * scale;
    const stroke = widget.ringThickness * scale;
    const radius = (size - stroke) / 2;
    const circumference = 2 * Math.PI * radius;
    const center = size / 2;
    const gradientId = `ring-grad-${metric}`;
    const glowId = `ring-glow-${metric}`;
    const useGradient = widget.ringStyle === "gradient";
    const sweep = Math.max(0, Math.min(1, ratio)) * circumference;
    const lineCap = isArcRingStyle(widget) ? "round" : widget.ringLineCap;

    if (!isArcRingStyle(widget)) {
        const trackDash = ringDashArray(widget, circumference, 1, true);
        const fillDash = ringDashArray(widget, circumference, ratio, false);

        return (
            <svg width={size} height={size} viewBox={`0 0 ${size} ${size}`} className="stat-widget-ring-svg">
                <circle
                    className="stat-widget-ring-track"
                    cx={center}
                    cy={center}
                    r={radius}
                    fill="none"
                    stroke={track}
                    strokeWidth={stroke}
                    strokeLinecap={lineCap}
                    strokeDasharray={trackDash}
                />
                <circle
                    className="stat-widget-ring-fill"
                    cx={center}
                    cy={center}
                    r={radius}
                    fill="none"
                    stroke={fill}
                    strokeWidth={stroke}
                    strokeLinecap={lineCap}
                    strokeDasharray={fillDash}
                    transform={`rotate(-90 ${center} ${center})`}
                />
            </svg>
        );
    }

    return (
        <svg width={size} height={size} viewBox={`0 0 ${size} ${size}`} className="stat-widget-ring-svg">
            <defs>
                {useGradient ? (
                    <linearGradient
                        id={gradientId}
                        gradientUnits="objectBoundingBox"
                        x1="0.5"
                        y1="0"
                        x2="0.5"
                        y2="1"
                        gradientTransform={`rotate(${widget.fillGradientAngle - 90} 0.5 0.5)`}
                    >
                        <stop offset="0%" stopColor={fill} />
                        <stop offset="100%" stopColor={colorToCss(widget.fillGradientColor)} />
                    </linearGradient>
                ) : null}
                <filter id={glowId} x="-35%" y="-35%" width="170%" height="170%">
                    <feGaussianBlur in="SourceGraphic" stdDeviation="1.6" result="blur" />
                    <feMerge>
                        <feMergeNode in="blur" />
                        <feMergeNode in="SourceGraphic" />
                    </feMerge>
                </filter>
            </defs>
            <circle
                className="stat-widget-ring-track-glass"
                cx={center}
                cy={center}
                r={radius}
                fill="none"
                stroke="rgba(255, 255, 255, 0.22)"
                strokeWidth={stroke}
                strokeLinecap="round"
            />
            <circle
                className="stat-widget-ring-track"
                cx={center}
                cy={center}
                r={radius}
                fill="none"
                stroke={track}
                strokeWidth={stroke}
                strokeLinecap="round"
                opacity={0.65}
            />
            <circle
                className="stat-widget-ring-fill"
                cx={center}
                cy={center}
                r={radius}
                fill="none"
                stroke={useGradient ? `url(#${gradientId})` : fill}
                strokeWidth={stroke}
                strokeLinecap="round"
                strokeDasharray={`${sweep} ${circumference}`}
                transform={`rotate(-90 ${center} ${center})`}
                filter={`url(#${glowId})`}
            />
        </svg>
    );
}

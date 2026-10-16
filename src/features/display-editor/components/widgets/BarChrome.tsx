import {
    barFillStyle,
    barTrackStyle,
    type OverlayWidgetConfig,
} from "../../../../entities/overlay";

interface BarChromeProps {
    widget: OverlayWidgetConfig;
    track: string;
    fill: string;
    ratio: number;
    scale: number;
}

export function BarChrome({ widget, track, fill, ratio, scale }: BarChromeProps) {
    const height = widget.barHeight * scale;
    const trackStyle = barTrackStyle(widget, track, scale);
    const fillStyle = {
        ...barFillStyle(widget, fill, scale),
        width: `${Math.round(ratio * 100)}%`,
    };

    return (
        <div className="stat-widget-bar-track" style={{ height, ...trackStyle }}>
            <div className="stat-widget-bar-fill" style={fillStyle} />
        </div>
    );
}

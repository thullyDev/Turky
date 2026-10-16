import {
    colorToHex,
    hexToColor,
    type OverlayWidgetConfig,
} from "../../../entities/overlay";

interface WidgetChromeFieldsProps {
    widget: OverlayWidgetConfig;
    onChange: (patch: Partial<OverlayWidgetConfig>) => void;
}

export function WidgetChromeFields({ widget, onChange }: WidgetChromeFieldsProps) {
    const showBar = widget.kind === "bar" || widget.kind === "stack";
    const showRing = widget.kind === "ring";

    if (widget.kind === "text") {
        return (
            <p className="editor-hint muted">
                Switch the widget to bar, ring, or stack to style progress chrome.
            </p>
        );
    }

    return (
        <>
            {showBar ? (
                <>
                    <label className="field">
                        <span className="muted">Bar width</span>
                        <input
                            type="number"
                            min={24}
                            value={widget.barWidth}
                            onChange={(event) =>
                                onChange({ barWidth: Number(event.target.value) })
                            }
                        />
                    </label>
                    <label className="field">
                        <span className="muted">Bar height</span>
                        <input
                            type="number"
                            min={2}
                            value={widget.barHeight}
                            onChange={(event) =>
                                onChange({ barHeight: Number(event.target.value) })
                            }
                        />
                    </label>
                    <label className="field">
                        <span className="muted">Corner radius</span>
                        <input
                            type="number"
                            min={0}
                            value={widget.barBorderRadius}
                            onChange={(event) =>
                                onChange({ barBorderRadius: Number(event.target.value) })
                            }
                        />
                    </label>
                </>
            ) : null}
            {showRing ? (
                <>
                    <label className="field">
                        <span className="muted">Ring size</span>
                        <input
                            type="number"
                            min={24}
                            value={widget.ringSize}
                            onChange={(event) =>
                                onChange({ ringSize: Number(event.target.value) })
                            }
                        />
                    </label>
                    <label className="field">
                        <span className="muted">Ring thickness</span>
                        <input
                            type="number"
                            min={2}
                            value={widget.ringThickness}
                            onChange={(event) =>
                                onChange({ ringThickness: Number(event.target.value) })
                            }
                        />
                    </label>
                </>
            ) : null}
            <label className="field">
                <span className="muted">Track</span>
                <input
                    type="color"
                    value={colorToHex(widget.trackColor)}
                    onChange={(event) =>
                        onChange({
                            trackColor: {
                                ...hexToColor(event.target.value),
                                a: widget.trackColor.a,
                            },
                        })
                    }
                />
            </label>
            <label className="field">
                <span className="muted">Fill</span>
                <input
                    type="color"
                    value={colorToHex(widget.fillColor)}
                    onChange={(event) =>
                        onChange({
                            fillColor: {
                                ...hexToColor(event.target.value),
                                a: widget.fillColor.a,
                            },
                        })
                    }
                />
            </label>
            {showBar ? (
                <>
                    <label className="field field-stack">
                        <span className="muted">Track CSS</span>
                        <textarea
                            className="editor-code"
                            rows={3}
                            spellCheck={false}
                            value={widget.barTrackCss}
                            onChange={(event) => onChange({ barTrackCss: event.target.value })}
                        />
                    </label>
                    <label className="field field-stack">
                        <span className="muted">Fill CSS</span>
                        <textarea
                            className="editor-code"
                            rows={3}
                            spellCheck={false}
                            value={widget.barFillCss}
                            onChange={(event) => onChange({ barFillCss: event.target.value })}
                        />
                    </label>
                </>
            ) : null}
            {showRing ? (
                <>
                    <label className="field field-stack">
                        <span className="muted">Track CSS</span>
                        <textarea
                            className="editor-code"
                            rows={3}
                            spellCheck={false}
                            placeholder="opacity: 0.8;"
                            value={widget.ringTrackCss}
                            onChange={(event) => onChange({ ringTrackCss: event.target.value })}
                        />
                    </label>
                    <label className="field field-stack">
                        <span className="muted">Fill CSS</span>
                        <textarea
                            className="editor-code"
                            rows={3}
                            spellCheck={false}
                            placeholder="filter: drop-shadow(...);"
                            value={widget.ringFillCss}
                            onChange={(event) => onChange({ ringFillCss: event.target.value })}
                        />
                    </label>
                </>
            ) : null}
        </>
    );
}

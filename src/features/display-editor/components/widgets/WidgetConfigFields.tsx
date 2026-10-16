import {
    colorToHex,
    hexToColor,
    WIDGET_KIND_OPTIONS,
    widgetPatchForKind,
    type OverlayWidgetConfig,
} from "../../../../entities/overlay";

interface WidgetConfigFieldsProps {
    widget: OverlayWidgetConfig;
    onChange: (patch: Partial<OverlayWidgetConfig>) => void;
}

export function WidgetConfigFields({ widget, onChange }: WidgetConfigFieldsProps) {
    const showBarFields = widget.kind === "bar" || widget.kind === "stack";
    const showRingFields = widget.kind === "ring";

    return (
        <>
            <label className="field">
                <span className="muted">Widget</span>
                <select
                    value={widget.kind}
                    onChange={(event) =>
                        onChange(
                            widgetPatchForKind(event.target.value as OverlayWidgetConfig["kind"]),
                        )
                    }
                >
                    {WIDGET_KIND_OPTIONS.map((option) => (
                        <option key={option.id} value={option.id}>
                            {option.label}
                        </option>
                    ))}
                </select>
            </label>
            <label className="field">
                <span className="muted">Label</span>
                <input
                    type="text"
                    placeholder="Default metric name"
                    value={widget.label}
                    onChange={(event) => onChange({ label: event.target.value })}
                />
            </label>
            <label className="field field-row">
                <input
                    type="checkbox"
                    checked={widget.showLabel}
                    onChange={(event) => onChange({ showLabel: event.target.checked })}
                />
                <span>Show label</span>
            </label>
            <label className="field field-row">
                <input
                    type="checkbox"
                    checked={widget.showValue}
                    onChange={(event) => onChange({ showValue: event.target.checked })}
                />
                <span>Show value</span>
            </label>
            {showBarFields ? (
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
                </>
            ) : null}
            {showRingFields ? (
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
            {widget.kind !== "text" ? (
                <>
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
                </>
            ) : null}
        </>
    );
}

import {
    WIDGET_KIND_OPTIONS,
    widgetPatchForKind,
    type OverlayWidgetConfig,
} from "../../../entities/overlay";

interface WidgetLayoutFieldsProps {
    widget: OverlayWidgetConfig;
    onChange: (patch: Partial<OverlayWidgetConfig>) => void;
}

export function WidgetLayoutFields({ widget, onChange }: WidgetLayoutFieldsProps) {
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
                <span className="muted">Label text</span>
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
        </>
    );
}

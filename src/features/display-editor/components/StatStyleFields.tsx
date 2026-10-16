import {
    FONT_PRESET_OPTIONS,
    FONT_WEIGHT_OPTIONS,
    type OverlayTextStyle,
} from "../../../entities/overlay";

interface StatStyleFieldsProps {
    style: OverlayTextStyle;
    onChange: (patch: Partial<OverlayTextStyle>) => void;
}

export function StatStyleFields({ style, onChange }: StatStyleFieldsProps) {
    return (
        <>
            <label className="field">
                <span className="muted">Font</span>
                <select
                    value={style.fontPreset}
                    onChange={(event) =>
                        onChange({
                            fontPreset: event.target.value as OverlayTextStyle["fontPreset"],
                        })
                    }
                >
                    {FONT_PRESET_OPTIONS.map((option) => (
                        <option key={option.id} value={option.id}>
                            {option.label}
                        </option>
                    ))}
                </select>
            </label>
            <label className="field">
                <span className="muted">Weight</span>
                <select
                    value={style.fontWeight}
                    onChange={(event) =>
                        onChange({
                            fontWeight: Number(event.target.value) as OverlayTextStyle["fontWeight"],
                        })
                    }
                >
                    {FONT_WEIGHT_OPTIONS.map((weight) => (
                        <option key={weight} value={weight}>
                            {weight}
                        </option>
                    ))}
                </select>
            </label>
            <label className="field field-row">
                <input
                    type="checkbox"
                    checked={style.italic}
                    onChange={(event) => onChange({ italic: event.target.checked })}
                />
                <span>Italic</span>
            </label>
            <label className="field field-row">
                <input
                    type="checkbox"
                    checked={style.uppercase}
                    onChange={(event) => onChange({ uppercase: event.target.checked })}
                />
                <span>Uppercase</span>
            </label>
            <label className="field">
                <span className="muted">Letter spacing</span>
                <input
                    type="number"
                    step={0.1}
                    value={style.letterSpacing}
                    onChange={(event) =>
                        onChange({ letterSpacing: Number(event.target.value) })
                    }
                />
            </label>
        </>
    );
}

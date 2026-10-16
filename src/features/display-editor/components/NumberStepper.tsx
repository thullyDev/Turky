interface NumberStepperProps {
    label: string;
    value: number;
    min: number;
    max: number;
    step?: number;
    onChange: (value: number) => void;
}

export function NumberStepper({
    label,
    value,
    min,
    max,
    step = 1,
    onChange,
}: NumberStepperProps) {
    const clamp = (next: number) => Math.min(max, Math.max(min, next));

    return (
        <label className="field field-stepper">
            <span className="muted">{label}</span>
            <div className="editor-stepper">
                <button
                    type="button"
                    className="editor-stepper-btn"
                    aria-label={`Decrease ${label}`}
                    onClick={() => onChange(clamp(value - step))}
                >
                    −
                </button>
                <input
                    type="range"
                    min={min}
                    max={max}
                    step={step}
                    value={value}
                    onChange={(event) => onChange(clamp(Number(event.target.value)))}
                />
                <input
                    type="number"
                    min={min}
                    max={max}
                    step={step}
                    value={value}
                    onChange={(event) => onChange(clamp(Number(event.target.value)))}
                />
                <button
                    type="button"
                    className="editor-stepper-btn"
                    aria-label={`Increase ${label}`}
                    onClick={() => onChange(clamp(value + step))}
                >
                    +
                </button>
            </div>
        </label>
    );
}

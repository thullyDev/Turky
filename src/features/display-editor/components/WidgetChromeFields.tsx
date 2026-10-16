import {
    BAR_CHROME_STYLE_OPTIONS,
    PROGRESS_KIND_HELP,
    RING_CHROME_STYLE_OPTIONS,
    RING_LINE_CAP_OPTIONS,
    WIDGET_KIND_OPTIONS,
    colorToHex,
    hexToColor,
    widgetPatchForKind,
    type OverlayWidgetConfig,
    type WidgetKind,
} from "../../../entities/overlay";
import { NumberStepper } from "./NumberStepper";

interface WidgetChromeFieldsProps {
    widget: OverlayWidgetConfig;
    onChange: (patch: Partial<OverlayWidgetConfig>) => void;
}

const PROGRESS_KINDS: WidgetKind[] = ["bar", "ring", "stack"];

export function WidgetChromeFields({ widget, onChange }: WidgetChromeFieldsProps) {
    const showBar = widget.kind === "bar" || widget.kind === "stack";
    const showRing = widget.kind === "ring";
    const onText = widget.kind === "text";

    return (
        <>
            <p className="editor-section-label">Progress type</p>
            <div className="editor-kind-pills" role="group" aria-label="Progress widget type">
                {WIDGET_KIND_OPTIONS.filter((option) => PROGRESS_KINDS.includes(option.id)).map(
                    (option) => (
                        <button
                            key={option.id}
                            type="button"
                            className={`editor-kind-pill${widget.kind === option.id ? " is-active" : ""}`}
                            onClick={() => onChange(widgetPatchForKind(option.id))}
                        >
                            {option.label}
                        </button>
                    ),
                )}
            </div>
            {widget.kind === "bar" || widget.kind === "ring" || widget.kind === "stack" ? (
                <p className="editor-hint muted">{PROGRESS_KIND_HELP[widget.kind]}</p>
            ) : null}
            {onText ? (
                <p className="editor-hint muted">
                    Choose bar, ring, or stack above, or click{" "}
                    <strong>Bar / ring / stack</strong> in the preview.
                </p>
            ) : null}
            {!onText ? (
                <>
                    {showBar ? (
                        <>
                            <p className="editor-section-label">Bar look</p>
                            <div className="editor-kind-pills" role="group" aria-label="Bar style">
                                {BAR_CHROME_STYLE_OPTIONS.map((option) => (
                                    <button
                                        key={option.id}
                                        type="button"
                                        className={`editor-kind-pill${widget.barStyle === option.id ? " is-active" : ""}`}
                                        onClick={() => onChange({ barStyle: option.id })}
                                    >
                                        {option.label}
                                    </button>
                                ))}
                            </div>
                            <NumberStepper
                                label="Width"
                                value={widget.barWidth}
                                min={24}
                                max={320}
                                step={4}
                                onChange={(barWidth) => onChange({ barWidth })}
                            />
                            <NumberStepper
                                label="Height"
                                value={widget.barHeight}
                                min={2}
                                max={48}
                                onChange={(barHeight) => onChange({ barHeight })}
                            />
                            <NumberStepper
                                label="Corner radius"
                                value={widget.barBorderRadius}
                                min={0}
                                max={999}
                                onChange={(barBorderRadius) => onChange({ barBorderRadius })}
                            />
                            {widget.barStyle === "dotted" ? (
                                <>
                                    <NumberStepper
                                        label="Dot size"
                                        value={widget.barDotSize}
                                        min={1}
                                        max={12}
                                        onChange={(barDotSize) => onChange({ barDotSize })}
                                    />
                                    <NumberStepper
                                        label="Dot gap"
                                        value={widget.barDotGap}
                                        min={2}
                                        max={24}
                                        onChange={(barDotGap) => onChange({ barDotGap })}
                                    />
                                </>
                            ) : null}
                            {widget.barStyle === "segmented" ? (
                                <>
                                    <NumberStepper
                                        label="Segment width"
                                        value={widget.barSegmentSize}
                                        min={4}
                                        max={40}
                                        onChange={(barSegmentSize) => onChange({ barSegmentSize })}
                                    />
                                    <NumberStepper
                                        label="Segment gap"
                                        value={widget.barSegmentGap}
                                        min={1}
                                        max={16}
                                        onChange={(barSegmentGap) => onChange({ barSegmentGap })}
                                    />
                                </>
                            ) : null}
                        </>
                    ) : null}
                    {showRing ? (
                        <>
                            <p className="editor-section-label">Ring look</p>
                            <div className="editor-kind-pills" role="group" aria-label="Ring style">
                                {RING_CHROME_STYLE_OPTIONS.map((option) => (
                                    <button
                                        key={option.id}
                                        type="button"
                                        className={`editor-kind-pill${widget.ringStyle === option.id ? " is-active" : ""}`}
                                        onClick={() => onChange({ ringStyle: option.id })}
                                    >
                                        {option.label}
                                    </button>
                                ))}
                            </div>
                            <NumberStepper
                                label="Ring size"
                                value={widget.ringSize}
                                min={24}
                                max={160}
                                step={2}
                                onChange={(ringSize) => onChange({ ringSize })}
                            />
                            <NumberStepper
                                label="Thickness"
                                value={widget.ringThickness}
                                min={2}
                                max={24}
                                onChange={(ringThickness) => onChange({ ringThickness })}
                            />
                            <label className="field">
                                <span className="muted">Line cap</span>
                                <select
                                    value={widget.ringLineCap}
                                    onChange={(event) =>
                                        onChange({
                                            ringLineCap: event.target
                                                .value as OverlayWidgetConfig["ringLineCap"],
                                        })
                                    }
                                >
                                    {RING_LINE_CAP_OPTIONS.map((option) => (
                                        <option key={option.id} value={option.id}>
                                            {option.label}
                                        </option>
                                    ))}
                                </select>
                            </label>
                            {widget.ringStyle === "segmented" ? (
                                <>
                                    <NumberStepper
                                        label="Segments"
                                        value={widget.ringSegmentCount}
                                        min={4}
                                        max={36}
                                        onChange={(ringSegmentCount) => onChange({ ringSegmentCount })}
                                    />
                                    <NumberStepper
                                        label="Segment gap"
                                        value={widget.ringSegmentGap}
                                        min={1}
                                        max={12}
                                        onChange={(ringSegmentGap) => onChange({ ringSegmentGap })}
                                    />
                                </>
                            ) : null}
                            {widget.ringStyle === "dashed" ? (
                                <>
                                    <NumberStepper
                                        label="Dash length"
                                        value={widget.ringDashLength}
                                        min={2}
                                        max={32}
                                        onChange={(ringDashLength) => onChange({ ringDashLength })}
                                    />
                                    <NumberStepper
                                        label="Dash gap"
                                        value={widget.ringDashGap}
                                        min={2}
                                        max={24}
                                        onChange={(ringDashGap) => onChange({ ringDashGap })}
                                    />
                                </>
                            ) : null}
                            {widget.ringStyle === "dotted" ? (
                                <>
                                    <NumberStepper
                                        label="Dot size"
                                        value={widget.ringDotSize}
                                        min={1}
                                        max={10}
                                        onChange={(ringDotSize) => onChange({ ringDotSize })}
                                    />
                                    <NumberStepper
                                        label="Dot gap"
                                        value={widget.ringDotGap}
                                        min={2}
                                        max={16}
                                        onChange={(ringDotGap) => onChange({ ringDotGap })}
                                    />
                                </>
                            ) : null}
                        </>
                    ) : null}
                    <p className="editor-section-label">Colors</p>
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
                    {widget.barStyle === "gradient" || widget.ringStyle === "gradient" ? (
                        <>
                            <label className="field">
                                <span className="muted">Gradient end</span>
                                <input
                                    type="color"
                                    value={colorToHex(widget.fillGradientColor)}
                                    onChange={(event) =>
                                        onChange({
                                            fillGradientColor: {
                                                ...hexToColor(event.target.value),
                                                a: widget.fillGradientColor.a,
                                            },
                                        })
                                    }
                                />
                            </label>
                            <NumberStepper
                                label="Gradient angle"
                                value={widget.fillGradientAngle}
                                min={0}
                                max={360}
                                step={5}
                                onChange={(fillGradientAngle) => onChange({ fillGradientAngle })}
                            />
                        </>
                    ) : null}
                    <p className="editor-section-label">Custom CSS</p>
                    <p className="editor-hint muted">
                        Override track, fill, or the whole progress shell. Preview only for glow,
                        filters, and gradients beyond the presets.
                    </p>
                    {showBar ? (
                        <>
                            <label className="field field-stack">
                                <span className="muted">Track CSS</span>
                                <textarea
                                    className="editor-code"
                                    rows={3}
                                    spellCheck={false}
                                    placeholder="box-shadow: inset 0 0 6px rgba(0,0,0,0.5);"
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
                                    placeholder="background: linear-gradient(90deg, #7c3aed, #22d3ee);"
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
                                    placeholder="opacity: 0.75;"
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
                                    placeholder="filter: drop-shadow(0 0 6px cyan);"
                                    value={widget.ringFillCss}
                                    onChange={(event) => onChange({ ringFillCss: event.target.value })}
                                />
                            </label>
                        </>
                    ) : null}
                    <label className="field field-stack">
                        <span className="muted">Shell CSS (whole progress)</span>
                        <textarea
                            className="editor-code"
                            rows={3}
                            spellCheck={false}
                            placeholder="filter: drop-shadow(0 0 8px rgba(56,189,248,0.5));"
                            value={widget.chromeCss}
                            onChange={(event) => onChange({ chromeCss: event.target.value })}
                        />
                    </label>
                </>
            ) : null}
        </>
    );
}

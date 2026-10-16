import { applyStylePreset, STYLE_PRESETS, type OverlayItem } from "../../../entities/overlay";

interface StylePresetPanelProps {
    item: OverlayItem;
    onReplace: (item: OverlayItem) => void;
}

export function StylePresetPanel({ item, onReplace }: StylePresetPanelProps) {
    return (
        <div className="editor-preset-grid">
            {STYLE_PRESETS.map((preset) => (
                <button
                    key={preset.id}
                    type="button"
                    className="editor-preset-card"
                    onClick={() => onReplace(applyStylePreset(item, preset.id))}
                >
                    <span className="editor-preset-name">{preset.name}</span>
                    <span className="editor-preset-desc muted">{preset.description}</span>
                </button>
            ))}
        </div>
    );
}

import {
    colorToHex,
    FONT_CATALOG,
    FONT_PRESET_OPTIONS,
    FONT_WEIGHT_OPTIONS,
    hexToColor,
    type TextBlockStyle,
} from "../../../entities/overlay";
import { NumberStepper } from "./NumberStepper";

interface TextBlockFieldsProps {
    block: TextBlockStyle;
    onChange: (patch: Partial<TextBlockStyle>) => void;
    onTypographyChange: (patch: Partial<TextBlockStyle["typography"]>) => void;
    customFamilies: Record<string, string>;
    isDownloaded: (fontId: string) => boolean;
    loadingFontId: string | null;
    fontError: string;
    onDownloadFont: (fontId: string) => void;
}

export function TextBlockFields({
    block,
    onChange,
    onTypographyChange,
    customFamilies,
    isDownloaded,
    loadingFontId,
    fontError,
    onDownloadFont,
}: TextBlockFieldsProps) {
    const usingCustom = Boolean(block.typography.customFontId);
    const customReady =
        block.typography.customFontId &&
        customFamilies[block.typography.customFontId];
    const gradient = block.gradient;

    return (
        <>
            <NumberStepper
                label="Size"
                value={block.size}
                min={6}
                max={96}
                onChange={(size) => onChange({ size })}
            />
            <label className="field">
                <span className="muted">Color</span>
                <input
                    type="color"
                    value={colorToHex(block.color)}
                    onChange={(event) =>
                        onChange({
                            color: {
                                ...hexToColor(event.target.value),
                                a: block.color.a,
                            },
                        })
                    }
                />
            </label>
            <label className="field field-row">
                <input
                    type="checkbox"
                    checked={gradient.enabled}
                    onChange={(event) =>
                        onChange({
                            gradient: { ...gradient, enabled: event.target.checked },
                        })
                    }
                />
                <span>Text gradient</span>
            </label>
            {gradient.enabled ? (
                <>
                    <label className="field">
                        <span className="muted">Gradient end</span>
                        <input
                            type="color"
                            value={colorToHex(gradient.endColor)}
                            onChange={(event) =>
                                onChange({
                                    gradient: {
                                        ...gradient,
                                        endColor: {
                                            ...hexToColor(event.target.value),
                                            a: gradient.endColor.a,
                                        },
                                    },
                                })
                            }
                        />
                    </label>
                    <NumberStepper
                        label="Gradient angle"
                        value={gradient.angle}
                        min={0}
                        max={360}
                        step={5}
                        onChange={(angle) =>
                            onChange({
                                gradient: { ...gradient, angle },
                            })
                        }
                    />
                </>
            ) : null}
            <NumberStepper
                label="Opacity"
                value={block.color.a}
                min={0}
                max={255}
                onChange={(alpha) =>
                    onChange({
                        color: {
                            ...block.color,
                            a: alpha,
                        },
                    })
                }
            />
            <label className="field">
                <span className="muted">Font</span>
                <select
                    value={usingCustom ? `custom:${block.typography.customFontId}` : block.typography.fontPreset}
                    onChange={(event) => {
                        const value = event.target.value;

                        if (value.startsWith("custom:")) {
                            onTypographyChange({
                                customFontId: value.slice("custom:".length),
                            });
                            return;
                        }

                        onTypographyChange({
                            fontPreset: value as TextBlockStyle["typography"]["fontPreset"],
                            customFontId: undefined,
                        });
                    }}
                >
                    {FONT_PRESET_OPTIONS.map((option) => (
                        <option key={option.id} value={option.id}>
                            {option.label}
                        </option>
                    ))}
                    {FONT_CATALOG.filter((entry) => isDownloaded(entry.id)).map((entry) => (
                        <option key={entry.id} value={`custom:${entry.id}`}>
                            {entry.label} (downloaded)
                        </option>
                    ))}
                </select>
            </label>
            {usingCustom && !customReady ? (
                <p className="editor-hint muted">Download this font below to preview it.</p>
            ) : null}
            <label className="field">
                <span className="muted">Weight</span>
                <select
                    value={block.typography.fontWeight}
                    onChange={(event) =>
                        onTypographyChange({
                            fontWeight: Number(
                                event.target.value,
                            ) as TextBlockStyle["typography"]["fontWeight"],
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
                    checked={block.typography.italic}
                    onChange={(event) => onTypographyChange({ italic: event.target.checked })}
                />
                <span>Italic</span>
            </label>
            <label className="field field-row">
                <input
                    type="checkbox"
                    checked={block.typography.uppercase}
                    onChange={(event) => onTypographyChange({ uppercase: event.target.checked })}
                />
                <span>Uppercase</span>
            </label>
            <NumberStepper
                label="Letter spacing"
                value={block.typography.letterSpacing}
                min={-2}
                max={4}
                step={0.1}
                onChange={(letterSpacing) => onTypographyChange({ letterSpacing })}
            />
            <label className="field field-stack">
                <span className="muted">Custom CSS</span>
                <textarea
                    className="editor-code"
                    rows={4}
                    spellCheck={false}
                    placeholder="text-shadow: 0 0 8px rgba(0,0,0,0.85);"
                    value={block.css}
                    onChange={(event) => onChange({ css: event.target.value })}
                />
            </label>
            <div className="editor-font-catalog">
                <p className="editor-section-label">Free fonts</p>
                <p className="editor-hint muted">Download only when you want to use one.</p>
                {fontError ? <p className="editor-hint editor-hint-error">{fontError}</p> : null}
                <ul className="editor-font-list">
                    {FONT_CATALOG.map((entry) => {
                        const downloaded = isDownloaded(entry.id);

                        return (
                            <li key={entry.id}>
                                <span>{entry.label}</span>
                                <button
                                    type="button"
                                    className="editor-font-download"
                                    disabled={downloaded || loadingFontId === entry.id}
                                    onClick={() => onDownloadFont(entry.id)}
                                >
                                    {downloaded
                                        ? "Ready"
                                        : loadingFontId === entry.id
                                          ? "…"
                                          : "Download"}
                                </button>
                            </li>
                        );
                    })}
                </ul>
            </div>
        </>
    );
}

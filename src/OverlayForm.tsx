import { defaultFont, overlayMetrics, textAnchors } from "./overlayConfig";
import type { OverlayItem, TextAnchor } from "./overlayConfig";
import { colorToHex, hexToColor } from "./overlayConfig";

interface OverlayFormProps {
    items: OverlayItem[];
    fonts: string[];
    onChange: (items: OverlayItem[]) => void;
    onRegisterFont: (name: string, bytes: number[]) => Promise<void>;
    onError: (message: string) => void;
}

function labelFor(metric: OverlayItem["metric"]): string {
    return overlayMetrics.find((entry) => entry.metric === metric)?.label ?? metric;
}

export default function OverlayForm({
    items,
    fonts,
    onChange,
    onRegisterFont,
    onError,
}: OverlayFormProps) {
    function update(metric: OverlayItem["metric"], patch: Partial<OverlayItem>) {
        onChange(
            items.map((item) => (item.metric === metric ? { ...item, ...patch } : item)),
        );
    }

    async function loadFont(event: React.ChangeEvent<HTMLInputElement>) {
        const file = event.target.files?.[0];

        if (!file) {
            return;
        }

        try {
            const buffer = await file.arrayBuffer();
            const bytes = Array.from(new Uint8Array(buffer));
            const name = file.name.replace(/\.(ttf|otf)$/i, "");

            await onRegisterFont(name, bytes);
        } catch (error) {
            console.error("Failed to load font:", error);
            onError("Failed to load font.");
        }
    }

    return (
        <section className="overlay-form">
            <h2>Overlays</h2>
            <p>Roboto is built in. Rust draws these onto each frame before it reaches the display.</p>

            {items.map((item) => {
                const fontsForItem = fonts.includes(item.font)
                    ? fonts
                    : [item.font, ...fonts];

                return (
                    <fieldset className="overlay-card" key={item.metric}>
                        <legend className="overlay-title">
                            <input
                                type="checkbox"
                                checked={item.enabled}
                                onChange={(event) =>
                                    update(item.metric, { enabled: event.target.checked })
                                }
                            />
                            {labelFor(item.metric)}
                        </legend>

                        <div className="overlay-grid">
                            <label>
                                X
                                <input
                                    type="number"
                                    value={item.x}
                                    onChange={(event) =>
                                        update(item.metric, { x: Number(event.target.value) })
                                    }
                                />
                            </label>
                            <label>
                                Y
                                <input
                                    type="number"
                                    value={item.y}
                                    onChange={(event) =>
                                        update(item.metric, { y: Number(event.target.value) })
                                    }
                                />
                            </label>
                            <label>
                                Anchor
                                <select
                                    value={item.anchor}
                                    onChange={(event) =>
                                        update(item.metric, {
                                            anchor: event.target.value as TextAnchor,
                                        })
                                    }
                                >
                                    {textAnchors.map((anchor) => (
                                        <option key={anchor.value} value={anchor.value}>
                                            {anchor.label}
                                        </option>
                                    ))}
                                </select>
                            </label>
                            <label>
                                Font
                                <select
                                    value={item.font}
                                    onChange={(event) =>
                                        update(item.metric, { font: event.target.value })
                                    }
                                >
                                    {fontsForItem.map((font) => (
                                        <option key={font} value={font}>
                                            {font === defaultFont ? `${defaultFont} (default)` : font}
                                        </option>
                                    ))}
                                </select>
                            </label>
                            <label>
                                Size
                                <input
                                    type="number"
                                    min={1}
                                    value={item.size}
                                    onChange={(event) =>
                                        update(item.metric, { size: Number(event.target.value) })
                                    }
                                />
                            </label>
                            <label>
                                Color
                                <input
                                    type="color"
                                    value={colorToHex(item.color)}
                                    onChange={(event) =>
                                        update(item.metric, {
                                            color: hexToColor(event.target.value),
                                        })
                                    }
                                />
                            </label>
                            {item.metric === "text" ? (
                                <label className="overlay-text">
                                    Text
                                    <input
                                        type="text"
                                        value={item.text}
                                        onChange={(event) =>
                                            update(item.metric, { text: event.target.value })
                                        }
                                    />
                                </label>
                            ) : null}
                        </div>
                    </fieldset>
                );
            })}

            <label className="media-picker">
                Font file
                <input type="file" accept=".ttf,.otf" onChange={loadFont} />
            </label>
        </section>
    );
}

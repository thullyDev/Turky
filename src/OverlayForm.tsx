import { colorToHex, hexToColor, overlayMetrics } from "./overlayConfig";
import type { OverlayItem } from "./overlayConfig";

interface OverlayFormProps {
    items: OverlayItem[];
    onChange: (items: OverlayItem[]) => void;
}

function labelFor(metric: OverlayItem["metric"]): string {
    return overlayMetrics.find((entry) => entry.metric === metric)?.label ?? metric;
}

export default function OverlayForm({ items, onChange }: OverlayFormProps) {
    function update(metric: OverlayItem["metric"], patch: Partial<OverlayItem>) {
        onChange(items.map((item) => (item.metric === metric ? { ...item, ...patch } : item)));
    }

    return (
        <section className="overlay-form">
            <h2>Stats</h2>
            <p>These are drawn onto each GIF frame.</p>

            {items.map((item) => (
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
                    </div>
                </fieldset>
            ))}
        </section>
    );
}

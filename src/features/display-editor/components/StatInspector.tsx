import {
    colorToHex,
    hexToColor,
    metricLabel,
    sampleStatLabel,
    statTextStyle,
    type OverlayItem,
} from "../../../entities/overlay";

interface StatInspectorProps {
    item: OverlayItem;
    onPatch: (patch: Partial<OverlayItem>) => void;
}

export function StatInspector({ item, onPatch }: StatInspectorProps) {
    return (
        <div className="editor-inspector">
            <p className="editor-inspector-title">{metricLabel(item.metric)}</p>
            <label className="field">
                <span className="muted">Size</span>
                <input
                    type="number"
                    min={1}
                    value={item.size}
                    onChange={(event) => onPatch({ size: Number(event.target.value) })}
                />
            </label>
            <label className="field">
                <span className="muted">Color</span>
                <input
                    type="color"
                    value={colorToHex(item.color)}
                    onChange={(event) =>
                        onPatch({
                            color: {
                                ...hexToColor(event.target.value),
                                a: item.color.a,
                            },
                        })
                    }
                />
            </label>
            <label className="field">
                <span className="muted">Opacity</span>
                <input
                    type="range"
                    min={0}
                    max={255}
                    value={item.color.a}
                    onChange={(event) =>
                        onPatch({
                            color: {
                                ...item.color,
                                a: Number(event.target.value),
                            },
                        })
                    }
                />
            </label>
            <div
                className="editor-stat-sample"
                style={statTextStyle(item, Math.min(1, 28 / item.size))}
            >
                {sampleStatLabel(item.metric)}
            </div>
        </div>
    );
}

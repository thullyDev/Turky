import {
    metricLabel,
    sampleStatLabel,
    statTextStyle,
    type OverlayItem,
    type OverlayMetric,
} from "../../../entities/overlay";

interface StatListProps {
    items: OverlayItem[];
    selected: OverlayMetric;
    onSelect: (metric: OverlayMetric) => void;
    onToggle: (metric: OverlayMetric, enabled: boolean) => void;
}

export function StatList({ items, selected, onSelect, onToggle }: StatListProps) {
    return (
        <div className="editor-stats">
            {items.map((item) => (
                <button
                    key={item.metric}
                    type="button"
                    className={`editor-stat-row${selected === item.metric ? " is-selected" : ""}`}
                    onClick={() => onSelect(item.metric)}
                >
                    <input
                        type="checkbox"
                        checked={item.enabled}
                        onChange={(event) => {
                            event.stopPropagation();
                            onToggle(item.metric, event.target.checked);
                        }}
                        onClick={(event) => event.stopPropagation()}
                    />
                    <span className="editor-stat-row-name">{metricLabel(item.metric)}</span>
                    <span
                        className="editor-stat-row-preview"
                        style={statTextStyle(item, 0.55)}
                        aria-hidden
                    >
                        {sampleStatLabel(item.metric)}
                    </span>
                </button>
            ))}
        </div>
    );
}

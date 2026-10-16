import type { PointerEvent, RefObject } from "react";

import { sampleStatLabel, statTextStyle, type OverlayItem, type OverlayMetric } from "../../../entities/overlay";

interface EditorStageProps {
    stageRef: RefObject<HTMLDivElement | null>;
    stageWidth: number;
    stageHeight: number;
    fitScale: number;
    mediaUrl: string | null;
    items: OverlayItem[];
    selected: OverlayMetric;
    dragging: OverlayMetric | null;
    onSelect: (metric: OverlayMetric) => void;
    onPointerMove: (event: PointerEvent) => void;
    onPointerUp: (event: PointerEvent) => void;
    onPointerCancel: (event: PointerEvent) => void;
    onStatPointerDown: (metric: OverlayMetric, event: PointerEvent) => void;
}

export function EditorStage({
    stageRef,
    stageWidth,
    stageHeight,
    fitScale,
    mediaUrl,
    items,
    selected,
    dragging,
    onSelect,
    onPointerMove,
    onPointerUp,
    onPointerCancel,
    onStatPointerDown,
}: EditorStageProps) {
    return (
        <div
            ref={stageRef}
            className="editor-stage"
            style={{ width: stageWidth, height: stageHeight }}
            onPointerMove={onPointerMove}
            onPointerUp={onPointerUp}
            onPointerCancel={onPointerCancel}
        >
            {mediaUrl ? (
                <img src={mediaUrl} alt="" className="editor-stage-media" draggable={false} />
            ) : (
                <div className="editor-stage-empty">
                    <span className="muted">No media</span>
                </div>
            )}

            {items
                .filter((item) => item.enabled)
                .map((item) => {
                    const isSelected = selected === item.metric;

                    return (
                        <button
                            key={item.metric}
                            type="button"
                            className={`editor-stat${isSelected ? " is-selected" : ""}${dragging === item.metric ? " is-dragging" : ""}`}
                            style={{
                                left: item.x * fitScale,
                                top: item.y * fitScale,
                                ...statTextStyle(item, fitScale),
                            }}
                            onPointerDown={(event) => onStatPointerDown(item.metric, event)}
                            onClick={() => onSelect(item.metric)}
                        >
                            {sampleStatLabel(item.metric)}
                        </button>
                    );
                })}
        </div>
    );
}

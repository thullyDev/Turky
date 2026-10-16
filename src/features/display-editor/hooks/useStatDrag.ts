import { useRef, useState, type RefObject } from "react";

import type { OverlayItem, OverlayMetric } from "../../../entities/overlay";

interface UseStatDragOptions {
    items: OverlayItem[];
    fitScale: number;
    displayWidth: number;
    displayHeight: number;
    stageRef: RefObject<HTMLDivElement | null>;
    onPatch: (metric: OverlayMetric, patch: Partial<OverlayItem>) => void;
    onSelect: (metric: OverlayMetric) => void;
}

function clampPosition(x: number, y: number, displayWidth: number, displayHeight: number) {
    return {
        x: Math.min(Math.max(0, Math.round(x)), Math.max(0, displayWidth - 8)),
        y: Math.min(Math.max(0, Math.round(y)), Math.max(0, displayHeight - 8)),
    };
}

export function useStatDrag({
    items,
    fitScale,
    displayWidth,
    displayHeight,
    stageRef,
    onPatch,
    onSelect,
}: UseStatDragOptions) {
    const [dragging, setDragging] = useState<OverlayMetric | null>(null);
    const dragOffset = useRef({ x: 0, y: 0 });

    function pointerToDevice(clientX: number, clientY: number) {
        const rect = stageRef.current?.getBoundingClientRect();

        if (!rect) {
            return { x: 0, y: 0 };
        }

        return {
            x: (clientX - rect.left) / fitScale,
            y: (clientY - rect.top) / fitScale,
        };
    }

    function beginDrag(metric: OverlayMetric, event: React.PointerEvent) {
        const item = items.find((entry) => entry.metric === metric);

        if (!item?.enabled) {
            return;
        }

        event.preventDefault();
        onSelect(metric);
        setDragging(metric);

        const pointer = pointerToDevice(event.clientX, event.clientY);

        dragOffset.current = {
            x: pointer.x - item.x,
            y: pointer.y - item.y,
        };

        event.currentTarget.setPointerCapture(event.pointerId);
    }

    function moveDrag(event: React.PointerEvent) {
        if (!dragging) {
            return;
        }

        const pointer = pointerToDevice(event.clientX, event.clientY);
        const next = clampPosition(
            pointer.x - dragOffset.current.x,
            pointer.y - dragOffset.current.y,
            displayWidth,
            displayHeight,
        );

        onPatch(dragging, next);
    }

    function endDrag(event: React.PointerEvent) {
        if (!dragging) {
            return;
        }

        if (event.currentTarget.hasPointerCapture(event.pointerId)) {
            event.currentTarget.releasePointerCapture(event.pointerId);
        }

        setDragging(null);
    }

    return { dragging, beginDrag, moveDrag, endDrag };
}

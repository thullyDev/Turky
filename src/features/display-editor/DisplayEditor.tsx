import { useEffect, useMemo, useRef, useState } from "react";

import type { OverlayItem, OverlayMetric } from "../../entities/overlay";
import type { DisplayInfo } from "../../shared/types/display";
import { useDownloadedFonts } from "../../shared/hooks/useDownloadedFonts";
import { useElementSize } from "../../shared/hooks/useElementSize";
import { fitScale as computeFitScale } from "../../shared/lib/fit-scale";
import { EditorSidebar } from "./components/EditorSidebar";
import { EditorStage } from "./components/EditorStage";
import { useDisplayInfo } from "./hooks/useDisplayInfo";
import { useDisplayPreview } from "./hooks/useDisplayPreview";
import { useEditorMedia } from "./hooks/useEditorMedia";
import { useOverlayItemPatch } from "./hooks/useOverlayItemPatch";
import { useStatDrag } from "./hooks/useStatDrag";
import { hasPreviewMedia } from "./lib/preview-media";
import "./styles/display-editor.css";

export interface DisplayEditorSaveContext {
    items: OverlayItem[];
    mediaBytes: ArrayBuffer | null;
    mediaType: string | null;
    display: DisplayInfo;
}

export interface DisplayEditorProps {
    items: OverlayItem[];
    onChange: (items: OverlayItem[]) => void;
    onStatus: (message: string) => void;
    status?: string;
    canvasDisplay?: Pick<DisplayInfo, "width" | "height"> | null;
    pendingMedia?: { bytes: ArrayBuffer; type: string } | null;
    onPendingMediaApplied?: () => void;
    autoSaveEnabled?: boolean;
    onAutoSaveDraft?: (context: DisplayEditorSaveContext) => void | Promise<void>;
    onContextChange?: (context: DisplayEditorSaveContext) => void;
}

const AUTO_SAVE_MS = 1400;

export function DisplayEditor({
    items,
    onChange,
    onStatus,
    status = "",
    canvasDisplay,
    pendingMedia,
    onPendingMediaApplied,
    autoSaveEnabled = true,
    onAutoSaveDraft,
    onContextChange,
}: DisplayEditorProps) {
    const stageRef = useRef<HTMLDivElement>(null);
    const { ref: paneRef, size: paneSize } = useElementSize<HTMLDivElement>();
    const liveDisplay = useDisplayInfo();
    const display = useMemo(
        () =>
            canvasDisplay
                ? { ...liveDisplay, width: canvasDisplay.width, height: canvasDisplay.height }
                : liveDisplay,
        [canvasDisplay, liveDisplay],
    );
    const patchItem = useOverlayItemPatch(items, onChange);
    const { mediaUrl, mediaBytes, mediaType, pickMedia, loadMedia } =
        useEditorMedia(() => onStatus(""));
    const { previewing, sendPreview } = useDisplayPreview(items, onStatus);
    const [selected, setSelected] = useState<OverlayMetric>("cpu");
    const fonts = useDownloadedFonts();
    const [hydrated, setHydrated] = useState(!pendingMedia);
    const skipNextAutoSave = useRef(true);

    const scale = computeFitScale(paneSize.w, paneSize.h, display);
    const stageWidth = display.width * scale;
    const stageHeight = display.height * scale;

    const { dragging, beginDrag, moveDrag, endDrag } = useStatDrag({
        items,
        fitScale: scale,
        displayWidth: display.width,
        displayHeight: display.height,
        stageRef,
        onPatch: patchItem,
        onSelect: setSelected,
    });

    const active = items.find((item) => item.metric === selected);
    const canPreview = hasPreviewMedia(mediaUrl, mediaBytes, mediaType);

    useEffect(() => {
        if (!pendingMedia) {
            setHydrated(true);
            return;
        }

        setHydrated(false);
        loadMedia(pendingMedia.bytes, pendingMedia.type);
        onPendingMediaApplied?.();
        setHydrated(true);
    }, [pendingMedia, loadMedia, onPendingMediaApplied]);

    function saveContext(): DisplayEditorSaveContext {
        return {
            items,
            mediaBytes,
            mediaType,
            display,
        };
    }

    useEffect(() => {
        onContextChange?.(saveContext());
    }, [items, mediaBytes, mediaType, display.width, display.height, onContextChange]);

    useEffect(() => {
        if (!autoSaveEnabled || !hydrated || !onAutoSaveDraft) {
            return;
        }

        if (skipNextAutoSave.current) {
            skipNextAutoSave.current = false;
            return;
        }

        const timer = window.setTimeout(() => {
            void onAutoSaveDraft(saveContext());
        }, AUTO_SAVE_MS);

        return () => window.clearTimeout(timer);
    }, [
        autoSaveEnabled,
        hydrated,
        onAutoSaveDraft,
        items,
        mediaBytes,
        mediaType,
        display.width,
        display.height,
    ]);

    return (
        <div className="editor">
            <div ref={paneRef} className="editor-preview">
                <EditorStage
                    stageRef={stageRef}
                    stageWidth={stageWidth}
                    stageHeight={stageHeight}
                    fitScale={scale}
                    mediaUrl={mediaUrl}
                    items={items}
                    selected={selected}
                    dragging={dragging}
                    customFamilies={fonts.families}
                    onSelect={setSelected}
                    onPointerMove={moveDrag}
                    onPointerUp={endDrag}
                    onPointerCancel={endDrag}
                    onStatPointerDown={beginDrag}
                />
            </div>

            <EditorSidebar
                display={display}
                status={status}
                items={items}
                selected={selected}
                active={active}
                previewScale={scale}
                previewing={previewing}
                canPreview={canPreview}
                customFamilies={fonts.families}
                isDownloaded={fonts.isDownloaded}
                loadingFontId={fonts.loadingId}
                fontError={fonts.error}
                onDownloadFont={(fontId) => void fonts.downloadFont(fontId)}
                onSelect={setSelected}
                onToggle={(metric, enabled) => patchItem(metric, { enabled })}
                onPatch={patchItem}
                onPickMedia={pickMedia}
                onPreview={() => void sendPreview(mediaUrl, mediaBytes, mediaType)}
            />
        </div>
    );
}

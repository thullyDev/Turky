import type { DisplayInfo } from "../../../shared/types/display";
import type { OverlayItem, OverlayMetric } from "../../../entities/overlay";
import { EditorToolbar } from "./EditorToolbar";
import { StatInspector } from "./StatInspector";
import { StatList } from "./StatList";

interface EditorSidebarProps {
    display: DisplayInfo;
    status: string;
    items: OverlayItem[];
    selected: OverlayMetric;
    active: OverlayItem | undefined;
    previewing: boolean;
    canPreview: boolean;
    onSelect: (metric: OverlayMetric) => void;
    onToggle: (metric: OverlayMetric, enabled: boolean) => void;
    onPatch: (metric: OverlayMetric, patch: Partial<OverlayItem>) => void;
    onPickMedia: (file: File | undefined) => void;
    onPreview: () => void;
}

export function EditorSidebar({
    display,
    status,
    items,
    selected,
    active,
    previewing,
    canPreview,
    onSelect,
    onToggle,
    onPatch,
    onPickMedia,
    onPreview,
}: EditorSidebarProps) {
    return (
        <aside className="editor-sidebar">
            <header className="editor-brand">
                <h1>TURKY</h1>
                <p className="muted">
                    {display.width}×{display.height}
                    {display.name ? ` · ${display.name}` : ""}
                </p>
            </header>

            <EditorToolbar
                previewing={previewing}
                canPreview={canPreview}
                onPickMedia={onPickMedia}
                onPreview={onPreview}
            />

            <StatList
                items={items}
                selected={selected}
                onSelect={onSelect}
                onToggle={onToggle}
            />

            {active ? <StatInspector item={active} onPatch={(patch) => onPatch(active.metric, patch)} /> : null}

            <p className="editor-status muted" aria-live="polite">
                {status || "Drag stats on the preview. Preview sends once."}
            </p>
        </aside>
    );
}

import { useMemo, useState } from "react";

import { metricLabel, type OverlayItem } from "../../../entities/overlay";
import { StylePresetPanel } from "./StylePresetPanel";
import { TextBlockFields } from "./TextBlockFields";
import { WidgetChromeFields } from "./WidgetChromeFields";
import { WidgetLayoutFields } from "./WidgetLayoutFields";
import { StatWidget } from "./widgets/StatWidget";

type InspectorTab = "widget" | "label" | "value" | "chrome" | "presets";

interface StatInspectorProps {
    item: OverlayItem;
    onPatch: (patch: Partial<OverlayItem>) => void;
    customFamilies: Record<string, string>;
    isDownloaded: (fontId: string) => boolean;
    loadingFontId: string | null;
    fontError: string;
    onDownloadFont: (fontId: string) => void;
}

export function StatInspector({
    item,
    onPatch,
    customFamilies,
    isDownloaded,
    loadingFontId,
    fontError,
    onDownloadFont,
}: StatInspectorProps) {
    const [tab, setTab] = useState<InspectorTab>("widget");

    const tabs = useMemo(() => {
        const entries: { id: InspectorTab; label: string }[] = [
            { id: "widget", label: "Widget" },
            { id: "label", label: "Label" },
            { id: "value", label: "Value" },
            { id: "chrome", label: "Bar / ring" },
            { id: "presets", label: "Presets" },
        ];

        return entries;
    }, []);

    const previewScale = Math.min(1, 28 / item.value.size);

    return (
        <div className="editor-inspector">
            <p className="editor-inspector-title">{metricLabel(item.metric)}</p>
            <div className="editor-tabs" role="tablist" aria-label="Stat inspector">
                {tabs.map((entry) => (
                    <button
                        key={entry.id}
                        type="button"
                        role="tab"
                        aria-selected={tab === entry.id}
                        className={`editor-tab${tab === entry.id ? " is-active" : ""}`}
                        onClick={() => setTab(entry.id)}
                    >
                        {entry.label}
                    </button>
                ))}
            </div>
            <div className="editor-tab-panel">
                {tab === "widget" ? (
                    <WidgetLayoutFields
                        widget={item.widget}
                        onChange={(patch) => onPatch({ widget: { ...item.widget, ...patch } })}
                    />
                ) : null}
                {tab === "label" ? (
                    <TextBlockFields
                        block={item.label}
                        onChange={(patch) => onPatch({ label: { ...item.label, ...patch } })}
                        onTypographyChange={(patch) =>
                            onPatch({
                                label: {
                                    ...item.label,
                                    typography: { ...item.label.typography, ...patch },
                                },
                            })
                        }
                        customFamilies={customFamilies}
                        isDownloaded={isDownloaded}
                        loadingFontId={loadingFontId}
                        fontError={fontError}
                        onDownloadFont={onDownloadFont}
                    />
                ) : null}
                {tab === "value" ? (
                    <TextBlockFields
                        block={item.value}
                        onChange={(patch) => onPatch({ value: { ...item.value, ...patch } })}
                        onTypographyChange={(patch) =>
                            onPatch({
                                value: {
                                    ...item.value,
                                    typography: { ...item.value.typography, ...patch },
                                },
                            })
                        }
                        customFamilies={customFamilies}
                        isDownloaded={isDownloaded}
                        loadingFontId={loadingFontId}
                        fontError={fontError}
                        onDownloadFont={onDownloadFont}
                    />
                ) : null}
                {tab === "chrome" ? (
                    <WidgetChromeFields
                        widget={item.widget}
                        onChange={(patch) => onPatch({ widget: { ...item.widget, ...patch } })}
                    />
                ) : null}
                {tab === "presets" ? (
                    <StylePresetPanel
                        item={item}
                        onReplace={(next) =>
                            onPatch({
                                label: next.label,
                                value: next.value,
                                widget: next.widget,
                            })
                        }
                    />
                ) : null}
            </div>
            <div className="editor-stat-sample">
                <StatWidget item={item} scale={previewScale} customFamilies={customFamilies} />
            </div>
        </div>
    );
}

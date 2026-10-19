import { useCallback, useEffect, useRef, useState } from "react";

import { createDefaultOverlayItems, normalizeOverlayItems, type OverlayItem } from "../entities/overlay";
import { AppSidebar } from "./components/AppSidebar";
import { DraftsHome, HomePage } from "../features/creator-portal";
import "../features/creator-portal/styles/creator-portal.css";
import { DisplayEditor, EditorChrome, type DisplayEditorSaveContext } from "../features/display-editor";
import { useMediaErrorListener } from "../shared/hooks/useMediaErrorListener";
import { hasNamedTitle } from "../shared/lib/creation-title";
import {
    getCreation,
    getCreationMedia,
    saveCreation,
} from "../shared/api/creation-api";
import "./styles/global.css";

const TITLE_AUTO_SAVE_MS = 1400;

type AppView = "home" | "drafts" | "editor";

interface PendingMedia {
    bytes: ArrayBuffer;
    type: string;
}

interface CanvasSize {
    width: number;
    height: number;
}

function copyMediaBuffer(media: number[]): ArrayBuffer {
    const bytes = new Uint8Array(media);
    return bytes.slice().buffer;
}

export function App() {
    const [view, setView] = useState<AppView>("home");
    const [message, setMessage] = useState("");
    const [saving, setSaving] = useState(false);
    const [creationId, setCreationId] = useState<string | null>(null);
    const [creationTitle, setCreationTitle] = useState("");
    const [items, setItems] = useState<OverlayItem[]>(createDefaultOverlayItems);
    const [pendingMedia, setPendingMedia] = useState<PendingMedia | null>(null);
    const [canvasDisplay, setCanvasDisplay] = useState<CanvasSize | null>(null);
    const [editorSession, setEditorSession] = useState(0);
    const [portalNotice, setPortalNotice] = useState("");
    const saveInFlight = useRef(false);

    useMediaErrorListener(useCallback((payload) => setMessage(payload), []));

    const persistDraft = useCallback(async (context: DisplayEditorSaveContext) => {
        if (saveInFlight.current) {
            return;
        }

        saveInFlight.current = true;
        setSaving(true);

        try {
            const summary = await saveCreation({
                id: creationId,
                title: creationTitle,
                kind: "draft",
                displayWidth: context.display.width,
                displayHeight: context.display.height,
                items,
                mediaBytes: context.mediaBytes,
                mediaType: context.mediaType,
            });

            setCreationId(summary.id);
            setCanvasDisplay({
                width: summary.displayWidth,
                height: summary.displayHeight,
            });
        } catch (error) {
            console.error("Auto-save failed", error);
            setMessage("Could not auto-save draft.");
        } finally {
            saveInFlight.current = false;
            setSaving(false);
        }
    }, [creationId, creationTitle, items]);

    const onAutoSaveDraft = useCallback(
        (context: DisplayEditorSaveContext) => persistDraft(context),
        [persistDraft],
    );

    async function persistSaved(context: DisplayEditorSaveContext) {
        if (!hasNamedTitle(creationTitle)) {
            setMessage("Name this creation before saving.");
            return;
        }

        setSaving(true);
        setMessage("");

        try {
            const summary = await saveCreation({
                id: creationId,
                title: creationTitle,
                kind: "saved",
                displayWidth: context.display.width,
                displayHeight: context.display.height,
                items,
                mediaBytes: context.mediaBytes,
                mediaType: context.mediaType,
            });

            setCreationId(summary.id);
            setCreationTitle(summary.title);
            setMessage("Saved to your library.");
        } catch (error) {
            console.error("Save failed", error);
            const detail = typeof error === "string" ? error : "Could not save creation.";
            setMessage(detail);
        } finally {
            setSaving(false);
        }
    }

    function startNewDraft() {
        setCreationId(null);
        setCreationTitle("");
        setItems(createDefaultOverlayItems());
        setPendingMedia(null);
        setCanvasDisplay(null);
        setMessage("");
        setEditorSession((value) => value + 1);
        setView("editor");
    }

    async function openCreation(id: string) {
        try {
            const document = await getCreation(id);
            const media = document.summary.hasMedia ? await getCreationMedia(id) : null;

            const rawItems = Array.isArray(document.items)
                ? (document.items as OverlayItem[])
                : createDefaultOverlayItems();

            setCreationId(document.summary.id);
            setCreationTitle(document.summary.title);
            setItems(normalizeOverlayItems(rawItems));
            setCanvasDisplay({
                width: document.summary.displayWidth,
                height: document.summary.displayHeight,
            });

            if (media && document.summary.mediaType) {
                setPendingMedia({
                    bytes: copyMediaBuffer(media),
                    type: document.summary.mediaType,
                });
            } else {
                setPendingMedia(null);
            }

            setEditorSession((value) => value + 1);
            setView("editor");
            setMessage("");
        } catch (error) {
            console.error("Open creation failed", error);
            setMessage("Could not open draft.");
        }
    }

    const editorSaveRef = useRef<DisplayEditorSaveContext | null>(null);
    const titleSaveSkip = useRef(true);

    const rememberEditorContext = useCallback((context: DisplayEditorSaveContext) => {
        editorSaveRef.current = context;
    }, []);

    useEffect(() => {
        if (view !== "editor") {
            titleSaveSkip.current = true;
            return;
        }

        if (titleSaveSkip.current) {
            titleSaveSkip.current = false;
            return;
        }

        const context = editorSaveRef.current;

        if (!context) {
            return;
        }

        const timer = window.setTimeout(() => {
            void persistDraft(context);
        }, TITLE_AUTO_SAVE_MS);

        return () => window.clearTimeout(timer);
    }, [creationTitle, view, persistDraft]);

    function handlePublishStub() {
        setMessage("Publish is coming soon.");
    }

    function stubPortal(message: string) {
        setPortalNotice(message);
        window.setTimeout(() => setPortalNotice(""), 2800);
    }

    return (
        <div className="app-shell">
            {view === "editor" ? (
                <div className="editor-workspace">
                    <EditorChrome
                        title={creationTitle}
                        saving={saving}
                        canSave={hasNamedTitle(creationTitle)}
                        onTitleChange={setCreationTitle}
                        onBack={() => setView("home")}
                        onSave={() => {
                            if (editorSaveRef.current) {
                                void persistSaved(editorSaveRef.current);
                            }
                        }}
                        onPublish={handlePublishStub}
                    />
                    <DisplayEditor
                        key={editorSession}
                        items={items}
                        onChange={setItems}
                        onStatus={setMessage}
                        status={message}
                        canvasDisplay={canvasDisplay}
                        pendingMedia={pendingMedia}
                        onPendingMediaApplied={() => setPendingMedia(null)}
                        onContextChange={rememberEditorContext}
                        onAutoSaveDraft={onAutoSaveDraft}
                    />
                </div>
            ) : (
                <div className="app-with-sidebar">
                    <AppSidebar
                        active={view === "drafts" ? "drafts" : "home"}
                        onHome={() => setView("home")}
                        onDrafts={() => setView("drafts")}
                        onSearchStub={() => stubPortal("Search is coming soon.")}
                        onUserStub={() => stubPortal("Account is coming soon.")}
                    />
                    <main className="app-main">
                        {portalNotice ? (
                            <p className="app-portal-notice muted" role="status">
                                {portalNotice}
                            </p>
                        ) : null}
                        {view === "home" ? (
                            <HomePage
                                onOpenCreation={(id) => void openCreation(id)}
                                onNewDraft={startNewDraft}
                                onMoreDrafts={() => setView("drafts")}
                            />
                        ) : (
                            <DraftsHome
                                onOpenCreation={(id) => void openCreation(id)}
                                onNewDraft={startNewDraft}
                                onBackHome={() => setView("home")}
                            />
                        )}
                    </main>
                </div>
            )}
        </div>
    );
}

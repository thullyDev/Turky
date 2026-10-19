import { DocumentTitleBar } from "./DocumentTitleBar";

interface EditorChromeProps {
    title: string;
    saving: boolean;
    canSave: boolean;
    onTitleChange: (title: string) => void;
    onBack: () => void;
    onSave: () => void;
    onPublish: () => void;
}

export function EditorChrome({
    title,
    saving,
    canSave,
    onTitleChange,
    onBack,
    onSave,
    onPublish,
}: EditorChromeProps) {
    return (
        <header className="editor-chrome">
            <button type="button" className="btn btn-ghost editor-chrome-back" onClick={onBack}>
                ← Drafts
            </button>

            <DocumentTitleBar title={title} onTitleChange={onTitleChange} saving={saving} />

            <div className="editor-chrome-actions">
                <button
                    type="button"
                    className="btn btn-ghost"
                    disabled={!canSave || saving}
                    title={canSave ? "Save to your library" : "Name this creation to save"}
                    onClick={onSave}
                >
                    Save
                </button>
                <button type="button" className="btn btn-primary" onClick={onPublish}>
                    Publish
                </button>
            </div>
        </header>
    );
}

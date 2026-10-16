interface EditorToolbarProps {
    previewing: boolean;
    canPreview: boolean;
    onPickMedia: (file: File | undefined) => void;
    onPreview: () => void;
}

export function EditorToolbar({
    previewing,
    canPreview,
    onPickMedia,
    onPreview,
}: EditorToolbarProps) {
    return (
        <div className="editor-actions">
            <label className="btn btn-ghost">
                Media
                <input
                    type="file"
                    accept="image/png,image/jpeg,image/webp,image/gif"
                    hidden
                    onChange={(event) => {
                        const file = event.target.files?.[0];
                        event.target.value = "";
                        void onPickMedia(file);
                    }}
                />
            </label>
            <button
                type="button"
                className="btn btn-primary"
                disabled={!canPreview || previewing}
                onClick={onPreview}
            >
                {previewing ? "Sending…" : "Preview"}
            </button>
        </div>
    );
}

import { useEffect, useRef, useState } from "react";

import {
    displayCreationTitle,
    hasNamedTitle,
    UNTITLED_DRAFT_LABEL,
} from "../../../shared/lib/creation-title";

interface DocumentTitleBarProps {
    title: string;
    onTitleChange: (title: string) => void;
    saving?: boolean;
}

export function DocumentTitleBar({ title, onTitleChange, saving }: DocumentTitleBarProps) {
    const [editing, setEditing] = useState(false);
    const [draft, setDraft] = useState(title);
    const inputRef = useRef<HTMLInputElement>(null);

    useEffect(() => {
        if (!editing) {
            setDraft(title);
        }
    }, [title, editing]);

    useEffect(() => {
        if (editing) {
            inputRef.current?.focus();
            inputRef.current?.select();
        }
    }, [editing]);

    function commit() {
        setEditing(false);
        onTitleChange(draft.trim());
    }

    const label = displayCreationTitle(title);
    const isUntitled = !hasNamedTitle(title);

    return (
        <div className="document-title-bar">
            {editing ? (
                <input
                    ref={inputRef}
                    className="document-title-input"
                    value={draft}
                    placeholder={UNTITLED_DRAFT_LABEL}
                    aria-label="Creation name"
                    onChange={(event) => setDraft(event.target.value)}
                    onBlur={commit}
                    onKeyDown={(event) => {
                        if (event.key === "Enter") {
                            event.preventDefault();
                            commit();
                        }

                        if (event.key === "Escape") {
                            setDraft(title);
                            setEditing(false);
                        }
                    }}
                />
            ) : (
                <button
                    type="button"
                    className={`document-title-display${isUntitled ? " is-placeholder" : ""}`}
                    onClick={() => setEditing(true)}
                >
                    {label}
                </button>
            )}
            {saving ? <span className="document-title-status muted">Saving…</span> : null}
        </div>
    );
}

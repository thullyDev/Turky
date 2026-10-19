import { convertFileSrc } from "@tauri-apps/api/core";

import type { CreationSummary } from "../../../entities/creation/types";
import { displayCreationTitle } from "../../../shared/lib/creation-title";

interface CreationCardProps {
    creation: CreationSummary;
    variant?: "grid" | "list" | "continue";
    onOpen: (id: string) => void;
    onDuplicate: (id: string) => void;
    onDelete: (id: string) => void;
}

function formatTimestamp(iso: string): string {
    const date = new Date(iso);

    if (Number.isNaN(date.getTime())) {
        return iso;
    }

    return date.toLocaleString(undefined, {
        dateStyle: "medium",
        timeStyle: "short",
    });
}

export function CreationCard({
    creation,
    variant = "grid",
    onOpen,
    onDuplicate,
    onDelete,
}: CreationCardProps) {
    const thumbSrc = creation.thumbnailPath
        ? convertFileSrc(creation.thumbnailPath)
        : null;

    const title = displayCreationTitle(creation.title);
    const edited = formatTimestamp(creation.updatedAt);

    if (variant === "continue") {
        return (
            <button
                type="button"
                className="creation-card creation-card-continue"
                onClick={() => onOpen(creation.id)}
            >
                <div className="creation-card-thumb creation-card-thumb-continue">
                    {thumbSrc ? (
                        <img src={thumbSrc} alt="" />
                    ) : (
                        <div className="creation-card-thumb-fallback" aria-hidden />
                    )}
                </div>
                <span className="creation-card-continue-title">{title}</span>
                <span className="creation-card-timestamp">{edited}</span>
            </button>
        );
    }

    if (variant === "list") {
        return (
            <article className="creation-card creation-card-list">
                <button
                    type="button"
                    className="creation-card-list-open"
                    onClick={() => onOpen(creation.id)}
                >
                    <div className="creation-card-thumb creation-card-thumb-sm">
                        {thumbSrc ? (
                            <img src={thumbSrc} alt="" />
                        ) : (
                            <div className="creation-card-thumb-fallback" aria-hidden />
                        )}
                    </div>
                    <div className="creation-card-list-text">
                        <h3>{title}</h3>
                        <p className="creation-card-timestamp">{edited}</p>
                        <p className="muted creation-card-meta">
                            {creation.displayWidth}×{creation.displayHeight}
                            {creation.hasMedia ? " · media" : " · overlay only"}
                        </p>
                    </div>
                </button>
                <div className="creation-card-actions creation-card-actions-compact">
                    <button
                        type="button"
                        className="btn btn-ghost"
                        onClick={() => onDuplicate(creation.id)}
                    >
                        Duplicate
                    </button>
                    <button
                        type="button"
                        className="btn btn-ghost creation-card-delete"
                        onClick={() => {
                            if (window.confirm(`Delete "${title}"?`)) {
                                onDelete(creation.id);
                            }
                        }}
                    >
                        Delete
                    </button>
                </div>
            </article>
        );
    }

    return (
        <article className="creation-card">
            <div className="creation-card-thumb">
                {thumbSrc ? (
                    <img src={thumbSrc} alt="" />
                ) : (
                    <div className="creation-card-thumb-fallback" aria-hidden />
                )}
            </div>

            <div className="creation-card-body">
                <h3>{title}</h3>
                <p className="creation-card-timestamp">{edited}</p>
                <p className="muted creation-card-meta">
                    {creation.displayWidth}×{creation.displayHeight}
                    {creation.hasMedia ? " · media attached" : " · overlay only"}
                </p>

                <div className="creation-card-actions">
                    <button type="button" className="btn btn-primary" onClick={() => onOpen(creation.id)}>
                        Open
                    </button>
                    <button
                        type="button"
                        className="btn btn-ghost"
                        onClick={() => onDuplicate(creation.id)}
                    >
                        Duplicate
                    </button>
                    <button
                        type="button"
                        className="btn btn-ghost creation-card-delete"
                        onClick={() => {
                            if (window.confirm(`Delete "${title}"?`)) {
                                onDelete(creation.id);
                            }
                        }}
                    >
                        Delete
                    </button>
                </div>
            </div>
        </article>
    );
}

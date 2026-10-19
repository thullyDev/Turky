import { CreationCard } from "./components/CreationCard";
import { useCreations } from "./hooks/useCreations";
import "./styles/creator-portal.css";

interface DraftsHomeProps {
    onOpenCreation: (id: string) => void;
    onNewDraft: () => void;
    onBackHome?: () => void;
}

export function DraftsHome({ onOpenCreation, onNewDraft, onBackHome }: DraftsHomeProps) {
    const { items, loading, error, remove, duplicate } = useCreations("draft");

    return (
        <div className="creator-portal drafts-page">
            <header className="creator-portal-header">
                <div>
                    {onBackHome ? (
                        <button type="button" className="home-section-more drafts-back" onClick={onBackHome}>
                            ← Home
                        </button>
                    ) : null}
                    <h1>Drafts</h1>
                    <p className="muted">All in-progress work. Untitled drafts auto-save as you edit.</p>
                </div>
                <button type="button" className="btn btn-primary" onClick={onNewDraft}>
                    New draft
                </button>
            </header>

            {loading ? <p className="muted creator-portal-status">Loading…</p> : null}
            {error ? <p className="creator-portal-error">{error}</p> : null}

            {!loading && !error && items.length === 0 ? (
                <div className="creator-portal-empty">
                    <p>No drafts yet.</p>
                    <p className="muted">Start a new draft — it saves automatically while you work.</p>
                    <button type="button" className="btn btn-primary" onClick={onNewDraft}>
                        New draft
                    </button>
                </div>
            ) : null}

            <ul className="drafts-list">
                {items.map((creation) => (
                    <li key={creation.id}>
                        <CreationCard
                            variant="list"
                            creation={creation}
                            onOpen={onOpenCreation}
                            onDuplicate={(id) => void duplicate(id)}
                            onDelete={(id) => void remove(id)}
                        />
                    </li>
                ))}
            </ul>
        </div>
    );
}

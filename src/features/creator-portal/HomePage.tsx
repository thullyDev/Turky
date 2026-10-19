import { CreationCard } from "./components/CreationCard";
import { useCreations } from "./hooks/useCreations";
import "./styles/creator-portal.css";

const CONTINUE_LIMIT = 10;

interface HomePageProps {
    onOpenCreation: (id: string) => void;
    onNewDraft: () => void;
    onMoreDrafts: () => void;
}

export function HomePage({ onOpenCreation, onNewDraft, onMoreDrafts }: HomePageProps) {
    const { items, loading, error } = useCreations("draft");
    const continueDrafts = items.slice(0, CONTINUE_LIMIT);

    return (
        <div className="creator-portal home-page">
            <header className="home-page-header">
                <div>
                    <h1>Home</h1>
                    <p className="muted">Pick up where you left off or start something new.</p>
                </div>
                <button type="button" className="btn btn-primary" onClick={onNewDraft}>
                    New draft
                </button>
            </header>

            {loading ? <p className="muted creator-portal-status">Loading…</p> : null}
            {error ? <p className="creator-portal-error">{error}</p> : null}

            {!loading && !error ? (
                <section className="home-continue" aria-labelledby="home-continue-heading">
                    <div className="home-section-row">
                        <h2 id="home-continue-heading">Continue</h2>
                        {items.length > 0 ? (
                            <button type="button" className="home-section-more" onClick={onMoreDrafts}>
                                More
                            </button>
                        ) : null}
                    </div>

                    {continueDrafts.length > 0 ? (
                        <ul className="home-continue-track">
                            {continueDrafts.map((creation) => (
                                <li key={creation.id}>
                                    <CreationCard
                                        variant="continue"
                                        creation={creation}
                                        onOpen={onOpenCreation}
                                        onDuplicate={() => {}}
                                        onDelete={() => {}}
                                    />
                                </li>
                            ))}
                        </ul>
                    ) : null}
                </section>
            ) : null}
        </div>
    );
}

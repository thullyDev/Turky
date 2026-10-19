interface AppSidebarProps {
    active: "home" | "drafts";
    onHome: () => void;
    onDrafts: () => void;
    onSearchStub: () => void;
    onUserStub: () => void;
}

export function AppSidebar({
    active,
    onHome,
    onDrafts,
    onSearchStub,
    onUserStub,
}: AppSidebarProps) {
    return (
        <aside className="app-sidebar" aria-label="App navigation">
            <div className="app-sidebar-header">
                <button type="button" className="app-sidebar-brand" onClick={onHome}>
                    <img src="/turky-mascot.jpg" alt="" className="app-sidebar-logo" />
                    <span className="app-sidebar-brand-text">Turky</span>
                </button>
                <button
                    type="button"
                    className="app-sidebar-user"
                    aria-label="Account"
                    title="Account (coming soon)"
                    onClick={onUserStub}
                >
                    <UserIcon />
                </button>
            </div>

            <div className="app-sidebar-search">
                <SearchIcon />
                <input
                    type="search"
                    readOnly
                    placeholder="Search"
                    aria-label="Search"
                    onFocus={onSearchStub}
                    onClick={onSearchStub}
                />
            </div>

            <nav className="app-sidebar-nav">
                <button
                    type="button"
                    className={`app-sidebar-nav-item${active === "drafts" ? " is-active" : ""}`}
                    onClick={onDrafts}
                >
                    <DraftIcon />
                    Drafts
                </button>
            </nav>
        </aside>
    );
}

function UserIcon() {
    return (
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" aria-hidden>
            <circle cx="12" cy="8" r="4" stroke="currentColor" strokeWidth="1.75" />
            <path
                d="M5 20c0-3.314 3.134-6 7-6s7 2.686 7 6"
                stroke="currentColor"
                strokeWidth="1.75"
                strokeLinecap="round"
            />
        </svg>
    );
}

function SearchIcon() {
    return (
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" aria-hidden>
            <circle cx="11" cy="11" r="6" stroke="currentColor" strokeWidth="1.75" />
            <path d="M16 16l5 5" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" />
        </svg>
    );
}

function DraftIcon() {
    return (
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" aria-hidden>
            <path
                d="M7 4h7l5 5v11a1 1 0 0 1-1 1H7a1 1 0 0 1-1-1V5a1 1 0 0 1 1-1z"
                stroke="currentColor"
                strokeWidth="1.75"
                strokeLinejoin="round"
            />
            <path d="M14 4v5h5" stroke="currentColor" strokeWidth="1.75" strokeLinejoin="round" />
        </svg>
    );
}

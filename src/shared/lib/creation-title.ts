/** Shown in UI when the draft has no user-provided name (stored as empty string). */
export const UNTITLED_DRAFT_LABEL = "Untitled draft";

const LEGACY_UNTITLED = /^untitled(\s+(creation|draft))?$/i;

export function hasNamedTitle(title: string): boolean {
    const trimmed = title.trim();

    if (!trimmed) {
        return false;
    }

    return !LEGACY_UNTITLED.test(trimmed);
}

export function displayCreationTitle(title: string): string {
    return hasNamedTitle(title) ? title.trim() : UNTITLED_DRAFT_LABEL;
}
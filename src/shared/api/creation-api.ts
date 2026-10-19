import { invoke } from "@tauri-apps/api/core";

import type { CreationDocument, CreationKind, CreationSummary } from "../../entities/creation/types";
import type { OverlayItem } from "../../entities/overlay";

export interface SaveCreationInput {
    id?: string | null;
    title: string;
    kind: CreationKind;
    displayWidth: number;
    displayHeight: number;
    items: OverlayItem[];
    mediaType?: string | null;
    mediaBytes?: ArrayBuffer | null;
}

export function listCreations(kind?: CreationKind): Promise<CreationSummary[]> {
    return invoke<CreationSummary[]>("list_creations", { kind: kind ?? null });
}

export function getCreation(id: string): Promise<CreationDocument> {
    return invoke<CreationDocument>("get_creation", { id });
}

export function getCreationMedia(id: string): Promise<number[] | null> {
    return invoke<number[] | null>("get_creation_media", { id });
}

export async function saveCreation(input: SaveCreationInput): Promise<CreationSummary> {
    const media = input.mediaBytes ? Array.from(new Uint8Array(input.mediaBytes)) : null;

    return invoke<CreationSummary>("save_creation", {
        request: {
            id: input.id ?? null,
            title: input.title,
            kind: input.kind,
            displayWidth: input.displayWidth,
            displayHeight: input.displayHeight,
            items: input.items,
            mediaType: input.mediaType ?? null,
        },
        media,
    });
}

export function renameCreation(id: string, title: string): Promise<CreationSummary> {
    return invoke<CreationSummary>("rename_creation", {
        request: { id, title },
    });
}

export function duplicateCreation(id: string): Promise<CreationSummary> {
    return invoke<CreationSummary>("duplicate_creation", { id });
}

export function deleteCreation(id: string): Promise<void> {
    return invoke<void>("delete_creation", { id });
}

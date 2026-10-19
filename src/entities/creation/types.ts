export type CreationKind = "draft" | "saved";

export interface CreationSummary {
    id: string;
    title: string;
    kind: CreationKind;
    displayWidth: number;
    displayHeight: number;
    updatedAt: string;
    createdAt: string;
    hasMedia: boolean;
    mediaType: string | null;
    thumbnailPath: string | null;
}

export interface CreationDocument {
    summary: CreationSummary;
    items: unknown;
}

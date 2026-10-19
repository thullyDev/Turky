const PREVIEW_MEDIA_TYPES = new Set([
    "image/png",
    "image/jpeg",
    "image/webp",
    "image/gif",
]);

export function hasPreviewMedia(
    mediaUrl: string | null,
    mediaBytes: ArrayBuffer | null,
    mediaType: string | null,
): boolean {
    if (!mediaUrl || !mediaBytes || mediaBytes.byteLength === 0 || !mediaType) {
        return false;
    }

    return PREVIEW_MEDIA_TYPES.has(mediaType);
}

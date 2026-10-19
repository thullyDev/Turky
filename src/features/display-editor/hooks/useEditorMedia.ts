import { useCallback, useEffect, useState } from "react";

export function useEditorMedia(onClearStatus: () => void) {
    const [mediaUrl, setMediaUrl] = useState<string | null>(null);
    const [mediaBytes, setMediaBytes] = useState<ArrayBuffer | null>(null);
    const [mediaType, setMediaType] = useState<string | null>(null);

    useEffect(() => {
        return () => {
            if (mediaUrl) {
                URL.revokeObjectURL(mediaUrl);
            }
        };
    }, [mediaUrl]);

    const loadMedia = useCallback((buffer: ArrayBuffer, type: string) => {
        setMediaUrl((previous) => {
            if (previous) {
                URL.revokeObjectURL(previous);
            }

            const blob = new Blob([buffer], { type });
            return URL.createObjectURL(blob);
        });
        setMediaBytes(buffer);
        setMediaType(type || "application/octet-stream");
    }, []);

    async function pickMedia(file: File | undefined) {
        if (!file) {
            return;
        }

        const buffer = await file.arrayBuffer();
        loadMedia(buffer, file.type);
        onClearStatus();
    }

    function clearMedia() {
        setMediaUrl((previous) => {
            if (previous) {
                URL.revokeObjectURL(previous);
            }

            return null;
        });
        setMediaBytes(null);
        setMediaType(null);
    }

    return {
        mediaUrl,
        mediaBytes,
        mediaType,
        pickMedia,
        loadMedia,
        clearMedia,
    };
}

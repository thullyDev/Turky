import { useEffect, useState } from "react";

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

    async function pickMedia(file: File | undefined) {
        if (!file) {
            return;
        }

        const buffer = await file.arrayBuffer();

        setMediaUrl((previous) => {
            if (previous) {
                URL.revokeObjectURL(previous);
            }

            return URL.createObjectURL(file);
        });
        setMediaBytes(buffer);
        setMediaType(file.type);
        onClearStatus();
    }

    return {
        mediaUrl,
        mediaBytes,
        mediaType,
        pickMedia,
    };
}

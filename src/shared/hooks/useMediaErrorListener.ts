import { listen } from "@tauri-apps/api/event";
import { useEffect } from "react";

export function useMediaErrorListener(onError: (message: string) => void) {
    useEffect(() => {
        const unlisten = listen<string>("media-error", (event) => {
            onError(event.payload);
        });

        return () => {
            unlisten.then((stop) => stop());
        };
    }, [onError]);
}

import { useEffect, useState } from "react";

import { fetchDisplayInfo } from "../../../shared/api/display-api";
import { FALLBACK_DISPLAY, type DisplayInfo } from "../../../shared/types/display";

export function useDisplayInfo() {
    const [display, setDisplay] = useState<DisplayInfo>(FALLBACK_DISPLAY);

    useEffect(() => {
        void fetchDisplayInfo().then((info) => {
            if (info) {
                setDisplay(info);
            }
        });
    }, []);

    return display;
}

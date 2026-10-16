import type { DisplayInfo } from "../types/display";

export function fitScale(
    paneWidth: number,
    paneHeight: number,
    display: Pick<DisplayInfo, "width" | "height">,
): number {
    if (paneWidth <= 0 || paneHeight <= 0) {
        return 1;
    }

    return Math.min(paneWidth / display.width, paneHeight / display.height);
}

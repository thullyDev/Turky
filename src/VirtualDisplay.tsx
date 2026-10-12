import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useRef } from "react";

interface FrameEvent {
    width: number;
    height: number;
    jpeg: string;
}

interface ClipBegin {
    id: number;
    width: number;
    height: number;
    frame_count: number;
    delays_ms: number[];
}

interface ClipFrame {
    id: number;
    index: number;
    jpeg: string;
}

interface OverlayEvent {
    id: number;
    png: string;
    x: number;
    y: number;
    width: number;
    height: number;
}

interface OverlayPatch {
    bitmap: ImageBitmap;
    x: number;
    y: number;
    width: number;
    height: number;
}

const canvasStyle: React.CSSProperties = {
    position: "fixed",
    inset: 0,
    width: "100%",
    height: "100%",
    margin: 0,
    padding: 0,
    border: 0,
    display: "block",
    background: "transparent",
};

export function frameIndex(delays: number[], started: number, now: number): number {
    let cycle = 0;

    for (const delay of delays) {
        cycle += delay;
    }

    if (cycle <= 0 || delays.length === 0) {
        return 0;
    }

    let time = (now - started) % cycle;

    for (let index = 0; index < delays.length; index++) {
        const delay = delays[index];

        if (time < delay) {
            return index;
        }

        time -= delay;
    }

    return delays.length - 1;
}

function decodeBase64(value: string): Uint8Array {
    const binary = atob(value);
    const bytes = new Uint8Array(binary.length);

    for (let index = 0; index < binary.length; index++) {
        bytes[index] = binary.charCodeAt(index);
    }

    return bytes;
}

function bitmapFrom(bytes: Uint8Array, type: string): Promise<ImageBitmap> {
    const blob = new Blob([bytes], { type });

    return createImageBitmap(blob).catch(
        () =>
            new Promise<ImageBitmap>((resolve, reject) => {
                const url = URL.createObjectURL(blob);
                const image = new Image();

                image.onload = () => {
                    URL.revokeObjectURL(url);
                    createImageBitmap(image).then(resolve, reject);
                };

                image.onerror = () => {
                    URL.revokeObjectURL(url);
                    reject(new Error(`Could not decode ${type}`));
                };

                image.src = url;
            }),
    );
}

function closeBitmap(bitmap: ImageBitmap | undefined) {
    bitmap?.close();
}

export default function VirtualDisplay() {
    const gifRef = useRef<HTMLCanvasElement>(null);
    const overlayRef = useRef<HTMLCanvasElement>(null);

    useEffect(() => {
        document.documentElement.style.margin = "0";
        document.documentElement.style.padding = "0";
        document.documentElement.style.width = "100%";
        document.documentElement.style.height = "100%";
        document.documentElement.style.overflow = "hidden";
        document.documentElement.style.background = "#000";

        document.body.style.margin = "0";
        document.body.style.padding = "0";
        document.body.style.width = "100%";
        document.body.style.height = "100%";
        document.body.style.overflow = "hidden";
        document.body.style.background = "#000";

        const gifCanvas = gifRef.current;
        const overlayCanvas = overlayRef.current;

        if (!gifCanvas || !overlayCanvas) {
            return;
        }

        const gifContext = gifCanvas.getContext("2d", { alpha: false });
        const overlayContext = overlayCanvas.getContext("2d", { alpha: true });

        if (!gifContext || !overlayContext) {
            return;
        }

        let generation = 0;
        let clipId = 0;
        let frameWidth = 0;
        let frameHeight = 0;
        let delays: number[] = [];
        let bitmaps: Array<ImageBitmap | undefined> = [];
        let ready = 0;
        let finished = 0;
        const queued = new Map<number, ClipFrame[]>();
        let started = 0;
        let lastIndex = -1;
        let presented = 0;
        let raf = 0;
        let playing = false;
        let overlay: OverlayPatch | null = null;
        let alive = true;

        const unlisteners: Array<Promise<() => void>> = [];

        const sizeCanvases = () => {
            const width = frameWidth || Math.max(1, window.innerWidth);
            const height = frameHeight || Math.max(1, window.innerHeight);
            const changed = gifCanvas.width !== width || gifCanvas.height !== height;

            if (!changed) {
                return false;
            }

            gifCanvas.width = width;
            gifCanvas.height = height;
            overlayCanvas.width = width;
            overlayCanvas.height = height;
            gifContext.imageSmoothingEnabled = true;
            gifContext.imageSmoothingQuality = "high";
            overlayContext.imageSmoothingEnabled = true;
            overlayContext.imageSmoothingQuality = "high";

            return true;
        };

        const drawOverlay = () => {
            overlayContext.clearRect(0, 0, overlayCanvas.width, overlayCanvas.height);

            if (!overlay || frameWidth === 0 || frameHeight === 0) {
                return;
            }

            const scaleX = overlayCanvas.width / frameWidth;
            const scaleY = overlayCanvas.height / frameHeight;

            overlayContext.drawImage(
                overlay.bitmap,
                overlay.x * scaleX,
                overlay.y * scaleY,
                overlay.width * scaleX,
                overlay.height * scaleY,
            );
        };

        const drawFrame = (index: number) => {
            const bitmap = bitmaps[index];

            if (!bitmap) {
                return;
            }

            gifContext.drawImage(bitmap, 0, 0, gifCanvas.width, gifCanvas.height);
            lastIndex = index;
        };

        const tick = (now: number) => {
            if (!alive || !playing) {
                return;
            }

            if (bitmaps.length > 0 && ready === bitmaps.length) {
                const index = frameIndex(delays, started, now);

                if (index !== lastIndex) {
                    drawFrame(index);
                    presented += 1;
                }
            }

            if (playing) {
                raf = requestAnimationFrame(tick);
            }
        };

        const stopPlayback = () => {
            playing = false;
            cancelAnimationFrame(raf);
            raf = 0;

            for (const bitmap of bitmaps) {
                closeBitmap(bitmap);
            }

            bitmaps = [];
            delays = [];
            ready = 0;
            finished = 0;
            lastIndex = -1;
            started = 0;
            closeBitmap(overlay?.bitmap);
            overlay = null;
            overlayContext.clearRect(0, 0, overlayCanvas.width, overlayCanvas.height);
        };

        const startClock = (current: number) => {
            if (playing || ready === 0 || finished !== bitmaps.length) {
                return;
            }

            cancelAnimationFrame(raf);
            playing = true;
            raf = requestAnimationFrame((now) => {
                if (!alive || current !== generation || !playing) {
                    return;
                }

                started = now;
                lastIndex = -1;
                tick(now);
            });
        };

        const storeFrame = (event: ClipFrame) => {
            if (event.index >= bitmaps.length) {
                bitmaps.length = event.index + 1;
            }

            if (bitmaps[event.index]) {
                return;
            }

            const current = generation;

            void bitmapFrom(decodeBase64(event.jpeg), "image/jpeg")
                .then((bitmap) => {
                    if (!alive || current !== generation) {
                        bitmap.close();
                        return;
                    }

                    if (bitmaps[event.index]) {
                        bitmap.close();
                        return;
                    }

                    bitmaps[event.index] = bitmap;
                    ready += 1;
                    finished += 1;
                    sizeCanvases();
                    drawFrame(event.index);
                    startClock(current);
                })
                .catch((error: unknown) => {
                    console.error("Turky could not decode a GIF frame", error);

                    if (!alive || current !== generation) {
                        return;
                    }

                    finished += 1;
                    startClock(current);
                });
        };

        const beginClip = (event: ClipBegin) => {
            generation += 1;
            clipId = event.id;
            frameWidth = event.width;
            frameHeight = event.height;
            stopPlayback();
            delays = event.delays_ms ?? [];
            bitmaps = new Array(event.frame_count);
            ready = 0;
            finished = 0;
            sizeCanvases();
            gifContext.fillStyle = "#000";
            gifContext.fillRect(0, 0, gifCanvas.width, gifCanvas.height);

            const pending = queued.get(event.id) ?? [];
            queued.delete(event.id);

            for (const frame of pending) {
                storeFrame(frame);
            }
        };

        const acceptFrame = (event: ClipFrame) => {
            if (event.id !== clipId || bitmaps.length === 0) {
                const pending = queued.get(event.id) ?? [];
                pending.push(event);
                queued.set(event.id, pending);
                return;
            }

            storeFrame(event);
        };

        const showStill = (event: FrameEvent) => {
            generation += 1;
            const current = generation;

            frameWidth = event.width;
            frameHeight = event.height;
            stopPlayback();
            sizeCanvases();

            void bitmapFrom(decodeBase64(event.jpeg), "image/jpeg").then((bitmap) => {
                if (!alive || current !== generation) {
                    bitmap.close();
                    return;
                }

                bitmaps = [bitmap];
                ready = 1;
                drawFrame(0);
            });
        };

        const showOverlay = (event: OverlayEvent) => {
            if (event.id !== clipId) {
                return;
            }

            const current = generation;

            if (!event.png) {
                closeBitmap(overlay?.bitmap);
                overlay = null;
                drawOverlay();
                return;
            }

            void bitmapFrom(decodeBase64(event.png), "image/png").then((bitmap) => {
                if (!alive || current !== generation || event.id !== clipId) {
                    bitmap.close();
                    return;
                }

                closeBitmap(overlay?.bitmap);
                overlay = {
                    bitmap,
                    x: event.x,
                    y: event.y,
                    width: event.width,
                    height: event.height,
                };
                drawOverlay();
            });
        };

        const onResize = () => {
            if (!sizeCanvases()) {
                return;
            }

            lastIndex = -1;

            if (ready > 0 && bitmaps.length > 0) {
                const index = started === 0 ? 0 : frameIndex(delays, started, performance.now());

                drawFrame(Math.min(index, bitmaps.length - 1));
            }

            drawOverlay();
        };

        sizeCanvases();

        const report = window.setInterval(() => {
            const frames = presented;

            presented = 0;

            if (frames === 0) {
                return;
            }

            void invoke("note_presented", { frames }).catch(() => {
                presented += frames;
            });
        }, 1000);

        unlisteners.push(
            listen<ClipBegin>("virtual-clip", (event) => {
                beginClip(event.payload);
            }),
        );
        unlisteners.push(
            listen<ClipFrame>("virtual-clip-frame", (event) => {
                acceptFrame(event.payload);
            }),
        );
        unlisteners.push(
            listen<OverlayEvent>("virtual-overlay", (event) => {
                showOverlay(event.payload);
            }),
        );
        unlisteners.push(
            listen<FrameEvent>("virtual-frame", (event) => {
                showStill(event.payload);
            }),
        );
        unlisteners.push(
            listen("virtual-clear", () => {
                generation += 1;
                stopPlayback();
                sizeCanvases();
                gifContext.fillStyle = "#000";
                gifContext.fillRect(0, 0, gifCanvas.width, gifCanvas.height);
            }),
        );

        window.addEventListener("resize", onResize);

        return () => {
            alive = false;
            window.clearInterval(report);
            window.removeEventListener("resize", onResize);
            generation += 1;
            stopPlayback();

            for (const pending of unlisteners) {
                void pending.then((unlisten) => unlisten());
            }
        };
    }, []);

    return (
        <>
            <canvas ref={gifRef} style={{ ...canvasStyle, background: "#000" }} />
            <canvas ref={overlayRef} style={{ ...canvasStyle, pointerEvents: "none" }} />
        </>
    );
}

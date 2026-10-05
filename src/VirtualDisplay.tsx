import { useEffect, useRef, useState } from "react";
import { listen } from "@tauri-apps/api/event";

interface FrameEvent {
    width: number;
    height: number;
    jpeg: string;
}

interface GifEvent {
    gif: string;
}

interface OverlayEvent {
    width: number;
    height: number;
    png: string;
}

type Paint =
    | { kind: "frame"; width: number; height: number; jpeg: string }
    | { kind: "overlay"; width: number; height: number; png: string };

const stageStyle = {
    position: "fixed" as const,
    top: 0,
    left: 0,
    width: "100vw",
    height: "100vh",
    margin: 0,
    padding: 0,
    border: 0,
    display: "block",
};

export default function VirtualDisplay() {
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const [gifSrc, setGifSrc] = useState("");

    useEffect(() => {
        document.documentElement.style.margin = "0";
        document.documentElement.style.padding = "0";
        document.documentElement.style.width = "100%";
        document.documentElement.style.height = "100%";
        document.documentElement.style.overflow = "hidden";

        document.body.style.margin = "0";
        document.body.style.padding = "0";
        document.body.style.width = "100%";
        document.body.style.height = "100%";
        document.body.style.overflow = "hidden";

        let unlistenFrame: (() => void) | undefined;
        let unlistenGif: (() => void) | undefined;
        let unlistenOverlay: (() => void) | undefined;
        let frameId = 0;
        let latest: Paint | null = null;
        let painted = "";
        let decoding = false;

        const clearCanvas = () => {
            const canvas = canvasRef.current;
            const context = canvas?.getContext("2d");

            if (!canvas || !context) {
                return;
            }

            context.clearRect(0, 0, canvas.width, canvas.height);
        };

        const drawLatest = () => {
            const canvas = canvasRef.current;
            const frame = latest;

            if (!canvas || !frame || decoding) {
                frameId = requestAnimationFrame(drawLatest);
                return;
            }

            const token = frame.kind === "frame" ? frame.jpeg : frame.png;

            if (token === painted) {
                frameId = requestAnimationFrame(drawLatest);
                return;
            }

            const context = canvas.getContext("2d");

            if (!context) {
                frameId = requestAnimationFrame(drawLatest);
                return;
            }

            decoding = true;
            const image = new Image();

            image.onload = () => {
                decoding = false;

                const current = latest;
                const currentToken =
                    current?.kind === "frame"
                        ? current.jpeg
                        : current?.kind === "overlay"
                          ? current.png
                          : "";

                if (currentToken !== token) {
                    frameId = requestAnimationFrame(drawLatest);
                    return;
                }

                if (canvas.width !== frame.width || canvas.height !== frame.height) {
                    canvas.width = frame.width;
                    canvas.height = frame.height;
                }

                if (frame.kind === "overlay") {
                    context.clearRect(0, 0, canvas.width, canvas.height);
                }

                context.drawImage(image, 0, 0, frame.width, frame.height);
                painted = token;
                frameId = requestAnimationFrame(drawLatest);
            };

            image.onerror = () => {
                decoding = false;
                frameId = requestAnimationFrame(drawLatest);
            };

            image.src =
                frame.kind === "frame"
                    ? `data:image/jpeg;base64,${frame.jpeg}`
                    : `data:image/png;base64,${frame.png}`;
        };

        const setupListener = async () => {
            try {
                unlistenGif = await listen<GifEvent>("virtual-gif", (event) => {
                    latest = null;
                    painted = "";
                    clearCanvas();

                    const gif = event.payload.gif;

                    setGifSrc(gif ? `data:image/gif;base64,${gif}` : "");
                });

                unlistenOverlay = await listen<OverlayEvent>("virtual-overlay", (event) => {
                    latest = {
                        kind: "overlay",
                        width: event.payload.width,
                        height: event.payload.height,
                        png: event.payload.png,
                    };
                });

                unlistenFrame = await listen<FrameEvent>("virtual-frame", (event) => {
                    setGifSrc("");
                    latest = {
                        kind: "frame",
                        width: event.payload.width,
                        height: event.payload.height,
                        jpeg: event.payload.jpeg,
                    };
                });

                frameId = requestAnimationFrame(drawLatest);
            } catch (error) {
                console.error("Failed to register virtual display listeners:", error);
            }
        };

        setupListener();

        return () => {
            cancelAnimationFrame(frameId);

            if (unlistenFrame) {
                unlistenFrame();
            }

            if (unlistenGif) {
                unlistenGif();
            }

            if (unlistenOverlay) {
                unlistenOverlay();
            }
        };
    }, []);

    return (
        <>
            {gifSrc ? (
                <img
                    src={gifSrc}
                    alt=""
                    style={{ ...stageStyle, objectFit: "fill", zIndex: 0 }}
                />
            ) : null}
            <canvas
                ref={canvasRef}
                style={{ ...stageStyle, background: "transparent", zIndex: 1 }}
            />
        </>
    );
}

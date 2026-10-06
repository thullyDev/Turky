import { useEffect, useRef } from "react";
import { listen } from "@tauri-apps/api/event";

interface FrameEvent {
    width: number;
    height: number;
    jpeg: string;
}

export default function VirtualDisplay() {
    const canvasRef = useRef<HTMLCanvasElement>(null);

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

        let unlisten: (() => void) | undefined;
        let frameId = 0;
        let latest: FrameEvent | null = null;
        let painted = "";
        let decoding = false;

        const drawLatest = () => {
            const canvas = canvasRef.current;
            const frame = latest;

            if (!canvas || !frame || decoding || frame.jpeg === painted) {
                frameId = requestAnimationFrame(drawLatest);
                return;
            }

            const context = canvas.getContext("2d");

            if (!context) {
                frameId = requestAnimationFrame(drawLatest);
                return;
            }

            decoding = true;
            const jpeg = frame.jpeg;
            const image = new Image();

            image.onload = () => {
                decoding = false;

                if (latest?.jpeg !== jpeg) {
                    frameId = requestAnimationFrame(drawLatest);
                    return;
                }

                if (canvas.width !== frame.width || canvas.height !== frame.height) {
                    canvas.width = frame.width;
                    canvas.height = frame.height;
                }

                context.drawImage(image, 0, 0, frame.width, frame.height);
                painted = jpeg;
                frameId = requestAnimationFrame(drawLatest);
            };

            image.onerror = () => {
                decoding = false;
                frameId = requestAnimationFrame(drawLatest);
            };

            image.src = `data:image/jpeg;base64,${jpeg}`;
        };

        const setupListener = async () => {
            try {
                unlisten = await listen<FrameEvent>("virtual-frame", (event) => {
                    latest = event.payload;
                });

                frameId = requestAnimationFrame(drawLatest);
            } catch (error) {
                console.error(
                    "Failed to register virtual-frame listener:",
                    error,
                );
            }
        };

        setupListener();

        return () => {
            cancelAnimationFrame(frameId);

            if (unlisten) {
                unlisten();
            }
        };
    }, []);

    return (
        <canvas
            ref={canvasRef}
            style={{
                position: "fixed",
                top: 0,
                left: 0,
                width: "100vw",
                height: "100vh",
                margin: 0,
                padding: 0,
                border: 0,
                display: "block",
            }}
        />
    );
}
import { useEffect, useRef } from "react";
import { listen } from "@tauri-apps/api/event";

interface FrameEvent {
    width: number;
    height: number;
    pixels: number[];
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

        const setupListener = async () => {
            try {
                unlisten = await listen<FrameEvent>(
                    "virtual-frame",
                    (event) => {
                        console.log("🔥 RECEIVED FRAME");

                        const frame = event.payload;

                        console.log(
                            `Frame: ${frame.width}x${frame.height}, ${frame.pixels.length} bytes`,
                        );

                        const canvas = canvasRef.current;

                        if (!canvas) {
                            console.error("Canvas not found");
                            return;
                        }

                        canvas.width = frame.width;
                        canvas.height = frame.height;

                        const context = canvas.getContext("2d");

                        if (!context) {
                            console.error("Could not get canvas context");
                            return;
                        }

                        const pixels = new Uint8ClampedArray(frame.pixels);

                        const imageData = new ImageData(
                            pixels,
                            frame.width,
                            frame.height,
                        );

                        context.putImageData(imageData, 0, 0);

                        console.log("✅ Frame rendered");
                    },
                );

                console.log("virtual-frame listener registered");
            } catch (error) {
                console.error(
                    "Failed to register virtual-frame listener:",
                    error,
                );
            }
        };

        setupListener();

        return () => {
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

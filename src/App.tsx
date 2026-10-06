import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import "./App.css";
import OverlayForm from "./OverlayForm";
import { defaultOverlayItems, toOverlayConfig } from "./overlayConfig";
import type { OverlayItem } from "./overlayConfig";

function App() {
    const [items, setItems] = useState<OverlayItem[]>(defaultOverlayItems);
    const [message, setMessage] = useState("");

    async function sendMediaToRust(event: React.ChangeEvent<HTMLInputElement>) {
        const file = event.target.files?.[0];

        if (!file) {
            return;
        }

        try {
            await invoke("set_overlay_config", {
                config: toOverlayConfig(items),
            });

            const buffer = await file.arrayBuffer();
            const bytes = Array.from(new Uint8Array(buffer));

            if (file.type === "image/gif") {
                await invoke("render_gif", {
                    bytes,
                });

                console.log("GIF sent to Rust");
            } else {
                await invoke("render_image", {
                    bytes,
                });

                console.log("Image sent to Rust");
            }

            setMessage("Media sent with the current stats.");
        } catch (error) {
            console.error("Failed to render media:", error);
            setMessage("Failed to render media.");
        }
    }

    return (
        <main className="container">
            <h1>Turky</h1>

            <OverlayForm items={items} onChange={setItems} />

            <label className="media-picker">
                Media
                <input
                    type="file"
                    accept="image/png,image/jpeg,image/webp,image/gif"
                    onChange={sendMediaToRust}
                />
            </label>

            <p className="status">{message}</p>
        </main>
    );
}

export default App;

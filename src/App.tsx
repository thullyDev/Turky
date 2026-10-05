import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import "./App.css";
import OverlayForm from "./OverlayForm";
import { defaultFont, defaultOverlayItems, toOverlayConfig } from "./overlayConfig";
import type { OverlayItem } from "./overlayConfig";

function App() {
    const [items, setItems] = useState<OverlayItem[]>(defaultOverlayItems);
    const [fonts, setFonts] = useState<string[]>([defaultFont]);
    const [message, setMessage] = useState("");

    useEffect(() => {
        let cancelled = false;

        invoke<string[]>("list_fonts")
            .then((names) => {
                if (!cancelled && names.length > 0) {
                    setFonts(names.includes(defaultFont) ? names : [defaultFont, ...names]);
                }
            })
            .catch((error) => {
                console.error("Failed to list fonts:", error);
            });

        return () => {
            cancelled = true;
        };
    }, []);

    async function registerFont(name: string, bytes: number[]) {
        await invoke("register_font", { name, bytes });

        setFonts((current) =>
            current.includes(name) ? current : [...current, name].sort(),
        );
    }

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

            setMessage("Media sent with the current overlays.");
        } catch (error) {
            console.error("Failed to render media:", error);
            setMessage("Failed to render media.");
        }
    }

    return (
        <main className="container">
            <h1>Turky</h1>

            <OverlayForm
                items={items}
                fonts={fonts}
                onChange={setItems}
                onRegisterFont={registerFont}
                onError={setMessage}
            />

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

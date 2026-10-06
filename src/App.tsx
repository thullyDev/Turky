import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
    async function sendMediaToRust(
        event: React.ChangeEvent<HTMLInputElement>,
    ) {
        const file = event.target.files?.[0];

        if (!file) {
            return;
        }

        try {
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
        } catch (error) {
            console.error(
                "Failed to render media:",
                error,
            );
        }
    }

    return (
        <main className="container">
            <h1>Turky</h1>

            <input
                type="file"
                accept="image/png,image/jpeg,image/webp,image/gif"
                onChange={sendMediaToRust}
            />
        </main>
    );
}

export default App;
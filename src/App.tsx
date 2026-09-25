import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
    async function sendImageToRust(
        event: React.ChangeEvent<HTMLInputElement>,
    ) {
        const file = event.target.files?.[0];

        if (!file) {
            return;
        }

        try {
            const buffer = await file.arrayBuffer();
            const bytes = Array.from(new Uint8Array(buffer));

            await invoke("render_image", {
                bytes,
            });

            console.log("Image sent to Rust");
        } catch (error) {
            console.error("Failed to render image:", error);
        }
    }

    return (
        <main className="container">
            <h1>Turky</h1>

            <input
                type="file"
                accept="image/png,image/jpeg,image/webp"
                onChange={sendImageToRust}
            />
        </main>
    );
}

export default App;

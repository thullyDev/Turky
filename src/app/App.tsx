import { useCallback, useState } from "react";

import { createDefaultOverlayItems, type OverlayItem } from "../entities/overlay";
import { DisplayEditor } from "../features/display-editor";
import { useMediaErrorListener } from "../shared/hooks/useMediaErrorListener";
import "./styles/global.css";

export function App() {
    const [items, setItems] = useState<OverlayItem[]>(createDefaultOverlayItems);
    const [message, setMessage] = useState("");

    useMediaErrorListener(useCallback((payload) => setMessage(payload), []));

    return (
        <DisplayEditor
            items={items}
            onChange={setItems}
            onStatus={setMessage}
            status={message}
        />
    );
}

import React from "react";
import ReactDOM from "react-dom/client";

import { App } from "./app/App";
import { VirtualDisplay } from "./features/virtual-display";

const isVirtualDisplay =
    new URLSearchParams(window.location.search).has("virtual-display");

ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
        {isVirtualDisplay ? <VirtualDisplay /> : <App />}
    </React.StrictMode>,
);

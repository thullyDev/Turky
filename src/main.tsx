import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import VirtualDisplay from "./VirtualDisplay";

const isVirtualDisplay =
    new URLSearchParams(window.location.search).has("virtual-display");

ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
        {isVirtualDisplay ? <VirtualDisplay /> : <App />}
    </React.StrictMode>,
);
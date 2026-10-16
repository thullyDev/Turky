import { useEffect, useRef, useState } from "react";

export function useElementSize<T extends HTMLElement>() {
    const ref = useRef<T>(null);
    const [size, setSize] = useState({ w: 0, h: 0 });

    useEffect(() => {
        const node = ref.current;

        if (!node) {
            return;
        }

        const observer = new ResizeObserver((entries) => {
            const box = entries[0]?.contentRect;

            if (box) {
                setSize({ w: box.width, h: box.height });
            }
        });

        observer.observe(node);

        return () => observer.disconnect();
    }, []);

    return { ref, size };
}

import { useCallback, useEffect, useState } from "react";

import type { CreationKind, CreationSummary } from "../../../entities/creation/types";
import {
    deleteCreation,
    duplicateCreation,
    listCreations,
    renameCreation,
} from "../../../shared/api/creation-api";

export function useCreations(kind: CreationKind) {
    const [items, setItems] = useState<CreationSummary[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState("");

    const refresh = useCallback(async () => {
        setLoading(true);
        setError("");

        try {
            const next = await listCreations(kind);
            setItems(next);
        } catch (cause) {
            console.error("Failed to load creations", cause);
            setError("Could not load creations.");
        } finally {
            setLoading(false);
        }
    }, [kind]);

    useEffect(() => {
        void refresh();
    }, [refresh]);

    async function remove(id: string) {
        await deleteCreation(id);
        await refresh();
    }

    async function duplicate(id: string) {
        await duplicateCreation(id);
        await refresh();
    }

    async function rename(id: string, title: string) {
        await renameCreation(id, title);
        await refresh();
    }

    return {
        items,
        loading,
        error,
        refresh,
        remove,
        duplicate,
        rename,
    };
}

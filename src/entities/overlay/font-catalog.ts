export interface CatalogFont {
    id: string;
    label: string;
    family: string;
    url: string;
    license: string;
}

export const FONT_CATALOG: CatalogFont[] = [
    {
        id: "orbitron",
        label: "Orbitron",
        family: "Orbitron",
        url: "https://github.com/google/fonts/raw/main/ofl/orbitron/Orbitron%5Bwght%5D.ttf",
        license: "OFL (Google Fonts)",
    },
    {
        id: "rajdhani",
        label: "Rajdhani",
        family: "Rajdhani",
        url: "https://github.com/google/fonts/raw/main/ofl/rajdhani/Rajdhani-Bold.ttf",
        license: "OFL (Google Fonts)",
    },
    {
        id: "share-tech-mono",
        label: "Share Tech Mono",
        family: "Share Tech Mono",
        url: "https://github.com/google/fonts/raw/main/ofl/sharetechmono/ShareTechMono-Regular.ttf",
        license: "OFL (Google Fonts)",
    },
    {
        id: "exo-2",
        label: "Exo 2",
        family: "Exo 2",
        url: "https://github.com/google/fonts/raw/main/ofl/exo2/Exo2%5Bwght%5D.ttf",
        license: "OFL (Google Fonts)",
    },
];

export function catalogFontById(id: string): CatalogFont | undefined {
    return FONT_CATALOG.find((entry) => entry.id === id);
}

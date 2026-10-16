import type { CSSProperties } from "react";

import { colorToCss } from "./color";
import {
    fontFamilyForTypography,
    letterSpacingForPreset,
} from "./text-style";
import type { TextBlockStyle } from "./text-block";

export function textBlockStyle(
    block: TextBlockStyle,
    scale: number,
    customFamilies: Record<string, string>,
): CSSProperties {
    const typography = block.typography;
    const spacing = letterSpacingForPreset(typography.fontPreset, typography.letterSpacing) * scale;
    const base: CSSProperties = {
        fontFamily: fontFamilyForTypography(typography, customFamilies),
        fontSize: block.size * scale,
        fontWeight: typography.fontWeight,
        fontStyle: typography.italic ? "italic" : "normal",
        lineHeight: 1,
        letterSpacing: spacing,
        textTransform: typography.uppercase ? "uppercase" : "none",
        color: colorToCss(block.color),
    };

    if (block.gradient?.enabled) {
        const from = colorToCss(block.color);
        const to = colorToCss(block.gradient.endColor);

        return {
            ...base,
            color: "transparent",
            backgroundImage: `linear-gradient(${block.gradient.angle}deg, ${from}, ${to})`,
            WebkitBackgroundClip: "text",
            backgroundClip: "text",
            WebkitTextFillColor: "transparent",
        };
    }

    return base;
}

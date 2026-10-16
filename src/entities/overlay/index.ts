export { createDefaultOverlayItems } from "./defaults";
export { colorToCss, colorToHex, hexToColor } from "./color";
export { FONT_CATALOG, catalogFontById } from "./font-catalog";
export { metricLabel, OVERLAY_METRICS } from "./metrics";
export { normalizeOverlayItem, normalizeOverlayItems } from "./normalize-item";
export { resolveWidgetLabel, sampleForMetric, sampleStatLabel } from "./stat-value";
export { textBlockStyle } from "./stat-style";
export { applyStylePreset, STYLE_PRESETS } from "./style-presets";
export {
    createDefaultLabelBlock,
    createDefaultTextGradient,
    createDefaultValueBlock,
    type TextBlockGradient,
    type TextBlockStyle,
} from "./text-block";
export {
    createDefaultTextStyle,
    FONT_PRESET_OPTIONS,
    FONT_WEIGHT_OPTIONS,
    fontFamilyForPreset,
    fontFamilyForTypography,
    letterSpacingForPreset,
} from "./text-style";
export type { OverlayFontPreset, OverlayTextStyle } from "./text-style";
export {
    createDefaultWidgetConfig,
    widgetPatchForKind,
    WIDGET_KIND_OPTIONS,
} from "./widget-config";
export type { OverlayWidgetConfig, WidgetKind } from "./widget-config";
export {
    BAR_CHROME_STYLE_OPTIONS,
    PROGRESS_KIND_HELP,
    RING_CHROME_STYLE_OPTIONS,
    RING_LINE_CAP_OPTIONS,
    barFillStyle,
    barTrackStyle,
    ringDashArray,
    ringStrokeColor,
    ringUsesGradient,
    svgGradientLine,
} from "./widget-chrome";
export type { BarChromeStyle, RingChromeStyle, RingLineCap } from "./widget-chrome";
export { toRustOverlayConfig } from "./rust-config";
export type { OverlayColor, OverlayItem, OverlayMetric } from "./types";

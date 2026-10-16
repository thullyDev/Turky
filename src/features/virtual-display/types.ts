export interface FrameEvent {
    width: number;
    height: number;
    jpeg: string;
}

export interface ClipBegin {
    id: number;
    width: number;
    height: number;
    frame_count: number;
    delays_ms: number[];
}

export interface ClipFrame {
    id: number;
    index: number;
    jpeg: string;
}

export interface OverlayEvent {
    id: number;
    png: string;
    x: number;
    y: number;
    width: number;
    height: number;
}

export interface OverlayPatch {
    bitmap: ImageBitmap;
    x: number;
    y: number;
    width: number;
    height: number;
}

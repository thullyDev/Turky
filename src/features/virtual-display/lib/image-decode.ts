export function decodeBase64(value: string): Uint8Array {
    const binary = atob(value);
    const bytes = new Uint8Array(binary.length);

    for (let index = 0; index < binary.length; index++) {
        bytes[index] = binary.charCodeAt(index);
    }

    return bytes;
}

export function bitmapFrom(bytes: Uint8Array, type: string): Promise<ImageBitmap> {
    const blob = new Blob([bytes], { type });

    return createImageBitmap(blob).catch(
        () =>
            new Promise<ImageBitmap>((resolve, reject) => {
                const url = URL.createObjectURL(blob);
                const image = new Image();

                image.onload = () => {
                    URL.revokeObjectURL(url);
                    createImageBitmap(image).then(resolve, reject);
                };

                image.onerror = () => {
                    URL.revokeObjectURL(url);
                    reject(new Error(`Could not decode ${type}`));
                };

                image.src = url;
            }),
    );
}

export function closeBitmap(bitmap: ImageBitmap | undefined) {
    bitmap?.close();
}

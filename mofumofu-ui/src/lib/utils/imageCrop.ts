import Pica from 'pica';

export interface CroppedArea {
	x: number;
	y: number;
	width: number;
	height: number;
}

const pica = new Pica({
	features: ['wasm', 'cib', 'js'] // js: JavaScript fallback, wasm: WebAssembly, cib: createImageBitmap
});

/**
 * Creates a cropped image from the original image and crop area using Pica for high quality resizing
 */
export async function getCroppedImg(
	imageSrc: string,
	pixelCrop: CroppedArea,
	resizeOptions?: { width: number; height: number },
	quality: number = 0.9
): Promise<{ blob: Blob; url: string }> {
	const image = await createImage(imageSrc);

	// Create source canvas with cropped area
	const sourceCanvas = document.createElement('canvas');
	const sourceCtx = sourceCanvas.getContext('2d');

	if (!sourceCtx) {
		throw new Error('No 2d context');
	}

	// Set source canvas size to the crop area
	sourceCanvas.width = pixelCrop.width;
	sourceCanvas.height = pixelCrop.height;

	// Draw the cropped image onto the source canvas
	sourceCtx.drawImage(
		image,
		pixelCrop.x,
		pixelCrop.y,
		pixelCrop.width,
		pixelCrop.height,
		0,
		0,
		pixelCrop.width,
		pixelCrop.height
	);

	// Create destination canvas
	const destCanvas = document.createElement('canvas');

	// Use resize options if provided, otherwise keep original crop size
	if (resizeOptions) {
		destCanvas.width = resizeOptions.width;
		destCanvas.height = resizeOptions.height;
	} else {
		destCanvas.width = pixelCrop.width;
		destCanvas.height = pixelCrop.height;
	}

	// Use pica to process the image (provides better quality)
	await pica.resize(sourceCanvas, destCanvas, {
		quality: 3
	});

	// Convert canvas to blob as WebP or JPEG fallback
	return new Promise((resolve, reject) => {
		const isWebPSupported = destCanvas.toDataURL('image/webp').indexOf('data:image/webp') === 0;

		const mimeType = isWebPSupported ? 'image/webp' : 'image/jpeg';

		destCanvas.toBlob(
			(blob) => {
				if (!blob) {
					reject(new Error('Canvas is empty'));
					return;
				}
				// console.log(`Resized image size: ${(blob.size / 1024).toFixed(2)} KB`);
				const url = URL.createObjectURL(blob);
				resolve({ blob, url });
			},
			mimeType,
			quality
		);
	});
}

/**
 * Creates an Image object from a source URL
 */
function createImage(url: string): Promise<HTMLImageElement> {
	return new Promise((resolve, reject) => {
		const image = new Image();
		image.addEventListener('load', () => resolve(image));
		image.addEventListener('error', (error) => reject(error));
		image.setAttribute('crossOrigin', 'anonymous');
		image.src = url;
	});
}

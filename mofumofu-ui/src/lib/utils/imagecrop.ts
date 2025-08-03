import Pica from 'pica';

export interface CroppedArea {
	x: number;
	y: number;
	width: number;
	height: number;
}

export interface CropOptions {
	maxFileSizeMB?: number;
	resizeOptions?: { width: number; height: number };
	quality?: number;
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
	options: CropOptions = {}
): Promise<{ blob: Blob; url: string; cleanup: () => void }> {
	const { maxFileSizeMB = 10, resizeOptions, quality = 0.9 } = options;
	const image = await createImage(imageSrc);

	let sourceCanvas: HTMLCanvasElement | null = null;
	let destCanvas: HTMLCanvasElement | null = null;
	let objectUrl: string | null = null;

	try {
		// Create source canvas with cropped area
		sourceCanvas = document.createElement('canvas');
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
		destCanvas = document.createElement('canvas');

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

		// Convert canvas to blob with file size validation
		const result = await new Promise<{ blob: Blob; url: string }>((resolve, reject) => {
			const isWebPSupported = destCanvas!.toDataURL('image/webp').indexOf('data:image/webp') === 0;
			const mimeType = isWebPSupported ? 'image/webp' : 'image/jpeg';

			destCanvas!.toBlob(
				(blob) => {
					if (!blob) {
						reject(new Error('Canvas is empty'));
						return;
					}

					// Check file size limit
					const fileSizeMB = blob.size / (1024 * 1024);
					if (fileSizeMB > maxFileSizeMB) {
						reject(new Error(`File size ${fileSizeMB.toFixed(2)}MB exceeds limit of ${maxFileSizeMB}MB`));
						return;
					}

					objectUrl = URL.createObjectURL(blob);
					resolve({ blob, url: objectUrl });
				},
				mimeType,
				quality
			);
		});

		// Cleanup function to revoke object URL and clear memory
		const cleanup = () => {
			if (objectUrl) {
				URL.revokeObjectURL(objectUrl);
				objectUrl = null;
			}
		};

		return { ...result, cleanup };
	} finally {
		// Clean up canvases immediately
		if (sourceCanvas) {
			sourceCanvas.width = 0;
			sourceCanvas.height = 0;
		}
		if (destCanvas) {
			destCanvas.width = 0;
			destCanvas.height = 0;
		}
	}
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

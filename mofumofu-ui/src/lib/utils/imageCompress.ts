import Pica from 'pica';

export interface CompressOptions {
	maxFileSizeMB?: number;
	resizeOptions?: { width: number; height: number };
	quality?: number;
}

const pica = new Pica({
	features: ['wasm', 'cib', 'js'] // js: JavaScript fallback, wasm: WebAssembly, cib: createImageBitmap
});

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

/**
 * Compresses an image without cropping using Pica for high quality resizing
 */
export async function compressImage(
	file: File,
	options: CompressOptions = {}
): Promise<{ blob: Blob; url: string; cleanup: () => void }> {
	const { maxFileSizeMB = 4, resizeOptions = { width: 800, height: 600 }, quality = 0.8 } = options;
	
	// Create image from file
	const imageSrc = URL.createObjectURL(file);
	let sourceImage: HTMLImageElement | null = null;
	let sourceCanvas: HTMLCanvasElement | null = null;
	let destCanvas: HTMLCanvasElement | null = null;
	let objectUrl: string | null = null;

	try {
		sourceImage = await createImage(imageSrc);
		
		// Create source canvas with original image
		sourceCanvas = document.createElement('canvas');
		const sourceCtx = sourceCanvas.getContext('2d');

		if (!sourceCtx) {
			throw new Error('No 2d context');
		}

		// Set source canvas size to original image size
		sourceCanvas.width = sourceImage.width;
		sourceCanvas.height = sourceImage.height;

		// Draw the original image onto the source canvas
		sourceCtx.drawImage(sourceImage, 0, 0);

		// Create destination canvas with resize options
		destCanvas = document.createElement('canvas');
		
		// Calculate aspect ratio preserving dimensions
		const aspectRatio = sourceImage.width / sourceImage.height;
		if (aspectRatio > resizeOptions.width / resizeOptions.height) {
			// Image is wider
			destCanvas.width = resizeOptions.width;
			destCanvas.height = resizeOptions.width / aspectRatio;
		} else {
			// Image is taller
			destCanvas.width = resizeOptions.height * aspectRatio;
			destCanvas.height = resizeOptions.height;
		}

		// Use pica to resize the image (provides better quality)
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

		// Cleanup function to revoke object URLs and clear memory
		const cleanup = () => {
			if (objectUrl) {
				URL.revokeObjectURL(objectUrl);
				objectUrl = null;
			}
			if (imageSrc) {
				URL.revokeObjectURL(imageSrc);
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
		// Clean up temporary image src
		URL.revokeObjectURL(imageSrc);
	}
}

/**
 * Compresses an image from canvas using Pica for high quality resizing
 */
export async function compressFromCanvas(
	canvas: HTMLCanvasElement,
	options: CompressOptions = {}
): Promise<{ blob: Blob; url: string; cleanup: () => void }> {
	const { maxFileSizeMB = 4, resizeOptions, quality = 0.8 } = options;
	
	let destCanvas: HTMLCanvasElement | null = null;
	let objectUrl: string | null = null;

	try {
		// Create destination canvas
		destCanvas = document.createElement('canvas');

		// Use resize options if provided, otherwise keep original canvas size
		if (resizeOptions) {
			destCanvas.width = resizeOptions.width;
			destCanvas.height = resizeOptions.height;
		} else {
			destCanvas.width = canvas.width;
			destCanvas.height = canvas.height;
		}

		// Use pica to process the image (provides better quality)
		await pica.resize(canvas, destCanvas, {
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
		// Clean up destination canvas immediately
		if (destCanvas) {
			destCanvas.width = 0;
			destCanvas.height = 0;
		}
	}
}
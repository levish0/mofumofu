import { compressFromCanvas, type CompressOptions } from './imageCompress';

export interface CroppedArea {
	x: number;
	y: number;
	width: number;
	height: number;
}

export interface CropOptions extends CompressOptions {}

/**
 * Creates a cropped image from the original image and crop area using imageCompress for high quality resizing
 */
export async function getCroppedImg(
	imageSrc: string,
	pixelCrop: CroppedArea,
	options: CropOptions = {}
): Promise<{ blob: Blob; url: string; cleanup: () => void }> {
	const image = await createImage(imageSrc);

	let sourceCanvas: HTMLCanvasElement | null = null;

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

		// Use compressFromCanvas to handle compression and resizing
		return await compressFromCanvas(sourceCanvas, options);
	} finally {
		// Clean up source canvas immediately
		if (sourceCanvas) {
			sourceCanvas.width = 0;
			sourceCanvas.height = 0;
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

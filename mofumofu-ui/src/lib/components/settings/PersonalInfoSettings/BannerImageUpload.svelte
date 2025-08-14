<script lang="ts">
	import { Icon, Photo } from 'svelte-hero-icons';
	import ImageCropModal from '../../modal/ImageCropModal.svelte';
	import { useImageCrop } from './useImageCrop';
	import * as m from '../../../../paraglide/messages';

	interface Props {
		bannerImage: string | null;
		onUpdate: (data: { bannerImageFile: Blob; bannerImage: string }) => void;
	}

	let { bannerImage, onUpdate }: Props = $props();

	// No cache-busting needed for blob URLs since they're already unique

	let showCrop = $state(false);
	let tempImageSrc = $state('');
	let imageLoading = $state(true);
	let imageError = $state(false);
	let retryCount = $state(0);
	let retryTimer: ReturnType<typeof setTimeout> | null = null;

	const { cropImage, cleanupTempImage, handleFileRead } = useImageCrop();

	async function handleImageChange(event: Event) {
		const target = event.target as HTMLInputElement;
		const file = target.files?.[0];
		if (file && file.type.startsWith('image/')) {
			try {
				tempImageSrc = await handleFileRead(file);
				showCrop = true;
			} catch (error) {
				console.error('Failed to read image file:', error);
				alert('Failed to read image file. Please try again.');
			}
		}
		target.value = '';
	}

	async function handleCrop(data: { croppedAreaPixels: { x: number; y: number; width: number; height: number } }) {
		try {
			const { blob, url } = await cropImage(tempImageSrc, data, {
				maxFileSizeMB: 8,
				resizeOptions: { width: 1000, height: 250 },
				quality: 0.9
			});

			onUpdate({
				bannerImageFile: blob,
				bannerImage: url
			});
			tempImageSrc = '';
		} catch (error) {
			console.error('Error cropping banner image:', error);
			if (error instanceof Error) {
				alert(`Banner crop failed: ${error.message}`);
			}
		}
	}

	function handleCropCancel() {
		cleanupTempImage(tempImageSrc);
		tempImageSrc = '';
	}

	function handleImageLoad() {
		imageLoading = false;
		imageError = false;
	}

	function handleImageError() {
		// Only retry for server URLs, not blob URLs
		if (bannerImage && !bannerImage.startsWith('blob:') && retryCount < 3) {
			const retryDelay = Math.pow(2, retryCount + 1) * 1000;
			retryTimer = setTimeout(() => {
				retryCount++;
				imageLoading = true;
				imageError = false;
			}, retryDelay);
		} else {
			imageLoading = false;
			imageError = true;
		}
	}

	// Reset loading state and retry count when image URL changes
	$effect(() => {
		if (bannerImage && !bannerImage.startsWith('blob:')) {
			imageLoading = true;
			imageError = false;
			retryCount = 0;
			if (retryTimer) {
				clearTimeout(retryTimer);
				retryTimer = null;
			}
		} else if (bannerImage && bannerImage.startsWith('blob:')) {
			imageLoading = false; // Blob URLs load instantly
			imageError = false;
			retryCount = 0;
		}
	});
</script>

<div class="space-y-4">
	<h2 class="text-2xl font-semibold">{m.settings_banner_image()}</h2>
	<div class="group relative transition-all">
		<div class="bg-mofu-dark-800 relative aspect-[4/1] w-full overflow-hidden rounded-lg group-hover:opacity-75">
			{#if bannerImage && !imageError}
				<!-- Skeleton shimmer while loading (only for server URLs) -->
				{#if imageLoading && !bannerImage.startsWith('blob:')}
					<div class="shimmer absolute inset-0 rounded-lg"></div>
				{/if}
				<img
					src={bannerImage}
					alt="Banner preview"
					class="h-full w-full object-cover {imageLoading && !bannerImage.startsWith('blob:')
						? 'opacity-0'
						: 'opacity-100'} transition-opacity duration-200"
					onload={handleImageLoad}
					onerror={handleImageError}
				/>
			{:else if imageError}
				<!-- Error fallback -->
				<div class="flex h-full w-full items-center justify-center bg-gradient-to-r from-blue-400 to-purple-500">
					<span class="text-sm text-white opacity-75">Image failed to load</span>
				</div>
			{/if}
			
			{#if bannerImage || imageError}
				<label
					for="banner-upload"
					class="dark:text-mofu-dark-300 absolute inset-0 flex cursor-pointer items-center justify-center hover:text-gray-300"
				>
				</label>
			{:else}
				<label
					for="banner-upload"
					class="text-mofu-dark-300 flex h-full cursor-pointer flex-col items-center justify-center space-y-2"
				>
					<Icon src={Photo} class="h-10 w-10" />
					<span class="text-sm">{m.settings_banner_image_upload()}</span>
					<span class="text-xs">{m.settings_banner_image_recommended()}</span>
				</label>
			{/if}
		</div>
		<input id="banner-upload" type="file" accept="image/*" class="hidden" onchange={handleImageChange} />
	</div>
</div>

<ImageCropModal
	bind:isOpen={showCrop}
	imageSrc={tempImageSrc}
	aspectRatio={4}
	cropShape="rect"
	onCrop={handleCrop}
	onCancel={handleCropCancel}
/>

<script lang="ts">
	import { Icon, Photo } from 'svelte-hero-icons';
	import ImageCropModal from '../../modal/ImageCropModal.svelte';
	import { useImageCrop } from './useImageCrop';

	interface Props {
		bannerImage: string | null;
		onUpdate: (data: { bannerImageFile: Blob; bannerImage: string }) => void;
	}

	let { bannerImage, onUpdate }: Props = $props();
	
	// No cache-busting needed for blob URLs since they're already unique

	let showCrop = $state(false);
	let tempImageSrc = $state('');
	let imageLoading = $state(true);

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
				maxFileSizeMB: 5,
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
	}

	function handleImageError() {
		imageLoading = false;
	}

	// Reset loading state when image URL changes (only for non-blob URLs)
	$effect(() => {
		if (bannerImage && !bannerImage.startsWith('blob:')) {
			imageLoading = true;
		} else if (bannerImage && bannerImage.startsWith('blob:')) {
			imageLoading = false; // Blob URLs load instantly
		}
	});
</script>

<div class="space-y-4">
	<h2 class="text-2xl font-semibold">Banner Image</h2>
	<div class="group relative transition-all">
		<div class="bg-mofu-dark-800 aspect-[4/1] w-full overflow-hidden rounded-lg group-hover:opacity-75 relative">
			{#if bannerImage}
				<!-- Skeleton shimmer while loading (only for server URLs) -->
				{#if imageLoading && !bannerImage.startsWith('blob:')}
					<div class="absolute inset-0 shimmer rounded-lg"></div>
				{/if}
				<img 
					src={bannerImage} 
					alt="Banner preview" 
					class="h-full w-full object-cover {imageLoading && !bannerImage.startsWith('blob:') ? 'opacity-0' : 'opacity-100'} transition-opacity duration-200"
					onload={handleImageLoad}
					onerror={handleImageError}
				/>
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
					<span class="text-sm">Click to upload banner image</span>
					<span class="text-xs">Recommended: 1600x400px</span>
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

<script lang="ts">
	import { Icon, Camera } from 'svelte-hero-icons';
	import ImageCropModal from '../../modal/ImageCropModal.svelte';
	import { useImageCrop } from './useImageCrop';
	import * as m from '../../../../paraglide/messages';

	interface Props {
		profileImage: string | null;
		onUpdate: (data: { profileImageFile: Blob; profileImage: string }) => void;
	}

	let { profileImage, onUpdate }: Props = $props();

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
				maxFileSizeMB: 4,
				resizeOptions: { width: 400, height: 400 },
				quality: 0.9
			});

			onUpdate({
				profileImageFile: blob,
				profileImage: url
			});
			tempImageSrc = '';
		} catch (error) {
			console.error('Error cropping profile image:', error);
			if (error instanceof Error) {
				alert(`Profile crop failed: ${error.message}`);
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
		if (profileImage && !profileImage.startsWith('blob:') && retryCount < 3) {
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
		if (profileImage && !profileImage.startsWith('blob:')) {
			imageLoading = true;
			imageError = false;
			retryCount = 0;
			if (retryTimer) {
				clearTimeout(retryTimer);
				retryTimer = null;
			}
		} else if (profileImage && profileImage.startsWith('blob:')) {
			imageLoading = false; // Blob URLs load instantly
			imageError = false;
			retryCount = 0;
		}
	});
</script>

<div class="space-y-4">
	<h2 class="text-2xl font-semibold">{m.settings_profile_image()}</h2>
	<div class="flex items-center space-x-4">
		<div class="group relative transition-all">
			<div class="bg-mofu-dark-800 relative h-24 w-24 overflow-hidden rounded-full group-hover:opacity-75">
				{#if profileImage && !imageError}
					<!-- Skeleton shimmer while loading (only for server URLs) -->
					{#if imageLoading && !profileImage.startsWith('blob:')}
						<div class="shimmer absolute inset-0 rounded-full"></div>
					{/if}
					<img
						src={profileImage}
						alt="Profile preview"
						class="h-full w-full object-cover {imageLoading && !profileImage.startsWith('blob:')
							? 'opacity-0'
							: 'opacity-100'} transition-opacity duration-200"
						onload={handleImageLoad}
						onerror={handleImageError}
					/>
				{:else if imageError}
					<!-- Error fallback -->
					<div class="flex h-full w-full items-center justify-center bg-gray-200 dark:bg-gray-700">
						<span class="text-xs text-gray-600 dark:text-gray-400">Failed</span>
					</div>
				{/if}
				
				{#if profileImage || imageError}
					<label
						for="profile-upload"
						class="dark:text-mofu-dark-300 absolute inset-0 flex cursor-pointer items-center justify-center"
					>
					</label>
				{:else}
					<label
						for="profile-upload"
						class="dark:text-mofu-dark-300 flex h-full cursor-pointer items-center justify-center"
					>
						<Icon src={Camera} class="h-6 w-6" />
					</label>
				{/if}
			</div>
			<input id="profile-upload" type="file" accept="image/*" class="hidden" onchange={handleImageChange} />
		</div>
		<div class="text-mofu-dark-400 text-sm">
			<p>{m.settings_profile_image_recommended()}</p>
			<p>{m.settings_profile_image_max_size()}</p>
		</div>
	</div>
</div>

<ImageCropModal
	bind:isOpen={showCrop}
	imageSrc={tempImageSrc}
	aspectRatio={1}
	cropShape="round"
	onCrop={handleCrop}
	onCancel={handleCropCancel}
/>

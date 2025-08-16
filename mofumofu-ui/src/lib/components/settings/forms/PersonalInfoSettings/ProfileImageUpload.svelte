<script lang="ts">
	import { Icon, Camera } from 'svelte-hero-icons';
	import { useImageCrop } from './useImageCrop';
	import * as m from '../../../../../paraglide/messages';

	interface Props {
		profileImage: string | null;
		onUpdate: (data: { profileImageFile: Blob; profileImage: string }) => void;
		openImageCrop: (
			imageSrc: string,
			aspectRatio?: number,
			shape?: 'rect' | 'round',
			onComplete?: (data: any) => void
		) => void;
	}

	let { profileImage, onUpdate, openImageCrop }: Props = $props();

	// No cache-busting needed for blob URLs since they're already unique

	let imageLoading = $state(true);

	const { cropImage, cleanupTempImage, handleFileRead } = useImageCrop();

	async function handleImageChange(event: Event) {
		const target = event.target as HTMLInputElement;
		const file = target.files?.[0];
		if (file && file.type.startsWith('image/')) {
			try {
				const tempImageSrc = await handleFileRead(file);

				openImageCrop(
					tempImageSrc,
					1,
					'round',
					async (data: { croppedAreaPixels: { x: number; y: number; width: number; height: number } }) => {
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
							cleanupTempImage(tempImageSrc);
						} catch (error) {
							console.error('Error cropping profile image:', error);
							if (error instanceof Error) {
								alert(`Profile crop failed: ${error.message}`);
							}
							cleanupTempImage(tempImageSrc);
						}
					}
				);
			} catch (error) {
				console.error('Failed to read image file:', error);
				alert('Failed to read image file. Please try again.');
			}
		}
		target.value = '';
	}

	function handleImageLoad() {
		imageLoading = false;
	}

	// Reset loading state when image URL changes
	$effect(() => {
		if (profileImage) {
			imageLoading = true;
		}
	});
</script>

<div class="space-y-4">
	<h2 class="text-2xl font-semibold">{m.settings_profile_image()}</h2>
	<div class="flex items-center space-x-4">
		<div class="group relative transition-all">
			<div class="bg-mofu-dark-800 relative h-24 w-24 overflow-hidden rounded-full group-hover:opacity-75">
				{#if profileImage}
					<!-- Skeleton shimmer while loading -->
					{#if imageLoading}
						<div class="shimmer absolute inset-0 rounded-full"></div>
					{/if}
					<img
						src={profileImage}
						alt="Profile preview"
						class="h-full w-full object-cover {imageLoading
							? 'opacity-0'
							: 'opacity-100'} transition-opacity duration-200"
						onload={handleImageLoad}
					/>
				{/if}

				{#if profileImage}
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

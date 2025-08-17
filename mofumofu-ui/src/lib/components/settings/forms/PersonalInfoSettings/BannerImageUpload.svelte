<script lang="ts">
	import { Icon, Photo } from 'svelte-hero-icons';
	import { useImageCrop } from './useImageCrop';
	import * as m from '../../../../../paraglide/messages';

	interface Props {
		bannerImage: string | null;
		onUpdate: (data: { bannerImageFile: Blob; bannerImage: string }) => void;
		openImageCrop: (
			imageSrc: string,
			aspectRatio?: number,
			shape?: 'rect' | 'round',
			onComplete?: (data: any) => void
		) => void;
	}

	let { bannerImage, onUpdate, openImageCrop }: Props = $props();

	// No cache-busting needed for blob URLs since they're already unique

	let imageLoading = $state(true);

	const { cropImage, cleanupTempImage, handleFileRead } = useImageCrop();

	async function handleImageChange(event: Event) {
		const target = event.target as HTMLInputElement;
		const file = target.files?.[0];
		if (file && file.type.startsWith('image/')) {
			// 파일 크기 체크 (8MB 제한)
			const fileSizeMB = file.size / (1024 * 1024);
			if (fileSizeMB > 8) {
				alert(`파일 크기가 ${fileSizeMB.toFixed(2)}MB로 8MB 제한을 초과합니다.`);
				target.value = '';
				return;
			}

			try {
				// GIF 파일은 crop 없이 바로 업로드
				if (file.type === 'image/gif') {
					const url = URL.createObjectURL(file);
					onUpdate({
						bannerImageFile: file,
						bannerImage: url
					});
					target.value = '';
					return;
				}

				// 다른 이미지 형식은 crop 모달 열기
				const tempImageSrc = await handleFileRead(file);

				openImageCrop(
					tempImageSrc,
					4,
					'rect',
					async (data: { croppedAreaPixels: { x: number; y: number; width: number; height: number } }) => {
						try {
							const { blob, url } = await cropImage(tempImageSrc, data, {
								maxFileSizeMB: 8
							});

							onUpdate({
								bannerImageFile: blob,
								bannerImage: url
							});
							cleanupTempImage(tempImageSrc);
						} catch (error) {
							console.error('Error cropping banner image:', error);
							if (error instanceof Error) {
								alert(`Banner crop failed: ${error.message}`);
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
		if (bannerImage) {
			imageLoading = true;
		}
	});
</script>

<div class="space-y-4">
	<h2 class="text-2xl font-semibold">{m.settings_banner_image()}</h2>
	<div class="group relative transition-all">
		<div class="bg-mofu-dark-800 relative aspect-[4/1] w-full overflow-hidden rounded-lg group-hover:opacity-75">
			{#if bannerImage}
				<!-- Skeleton shimmer while loading -->
				{#if imageLoading}
					<div class="shimmer absolute inset-0 rounded-lg"></div>
				{/if}
				<img
					src={bannerImage}
					alt="Banner preview"
					class="h-full w-full object-cover {imageLoading
						? 'opacity-0'
						: 'opacity-100'} transition-opacity duration-200"
					onload={handleImageLoad}
				/>
			{/if}

			{#if bannerImage}
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

<script lang="ts">
	import { Icon, Photo } from 'svelte-hero-icons';
	import ImageCropModal from '../../modal/ImageCropModal.svelte';
	import { useImageCrop } from '../../settings/PersonalInfoSettings/useImageCrop';

	interface Props {
		thumbnail: string | null;
		onUpdate: (data: { thumbnailFile: Blob; thumbnail: string } | null) => void;
	}

	let { thumbnail, onUpdate }: Props = $props();

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
				alert('이미지 파일을 읽는데 실패했습니다. 다시 시도해주세요.');
			}
		}
		target.value = '';
	}

	async function handleCrop(data: { croppedAreaPixels: { x: number; y: number; width: number; height: number } }) {
		try {
			const { blob, url } = await cropImage(tempImageSrc, data, {
				maxFileSizeMB: 3,
				resizeOptions: { width: 800, height: 450 }, // 16:9 비율
				quality: 0.85
			});

			onUpdate({
				thumbnailFile: blob,
				thumbnail: url
			});
			tempImageSrc = '';
		} catch (error) {
			console.error('Error cropping thumbnail:', error);
			if (error instanceof Error) {
				alert(`썸네일 크롭 실패: ${error.message}`);
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

	function removeThumbnail() {
		if (thumbnail && thumbnail.startsWith('blob:')) {
			URL.revokeObjectURL(thumbnail);
		}
		onUpdate(null);
	}

	// Reset loading state when image URL changes
	$effect(() => {
		if (thumbnail && !thumbnail.startsWith('blob:')) {
			imageLoading = true;
		} else if (thumbnail && thumbnail.startsWith('blob:')) {
			imageLoading = false;
		}
	});
</script>

<div class="space-y-4">
	<h2 class="text-lg font-medium text-mofu-dark-100">썸네일 (선택사항)</h2>
	<div class="group relative transition-all">
		<div class="bg-mofu-dark-800 relative aspect-video w-full overflow-hidden rounded-lg group-hover:opacity-75">
			{#if thumbnail}
				<!-- Loading shimmer -->
				{#if imageLoading && !thumbnail.startsWith('blob:')}
					<div class="shimmer absolute inset-0 rounded-lg"></div>
				{/if}
				<img
					src={thumbnail}
					alt="썸네일 미리보기"
					class="h-full w-full object-cover {imageLoading && !thumbnail.startsWith('blob:')
						? 'opacity-0'
						: 'opacity-100'} transition-opacity duration-200"
					onload={handleImageLoad}
					onerror={handleImageError}
				/>
				<label
					for="thumbnail-upload"
					class="dark:text-mofu-dark-300 absolute inset-0 flex cursor-pointer items-center justify-center hover:text-gray-300"
				>
				</label>
			{:else}
				<label
					for="thumbnail-upload"
					class="text-mofu-dark-300 flex h-full cursor-pointer flex-col items-center justify-center space-y-2"
				>
					<Icon src={Photo} class="h-10 w-10" />
					<span class="text-sm">썸네일 이미지 업로드</span>
					<span class="text-xs">16:9 비율 권장, 최대 3MB</span>
				</label>
			{/if}
		</div>
		<input id="thumbnail-upload" type="file" accept="image/*" class="hidden" onchange={handleImageChange} />
	</div>
	{#if thumbnail}
		<button onclick={removeThumbnail} class="text-red-400 hover:text-red-300 text-xs underline">
			썸네일 제거
		</button>
	{/if}
</div>

<ImageCropModal
	bind:isOpen={showCrop}
	imageSrc={tempImageSrc}
	aspectRatio={16 / 9}
	cropShape="rect"
	onCrop={handleCrop}
	onCancel={handleCropCancel}
/>

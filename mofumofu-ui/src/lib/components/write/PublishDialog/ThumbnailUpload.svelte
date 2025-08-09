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

<div>
	<label class="text-mofu-dark-100 mb-2 block text-sm font-medium">썸네일 (선택사항)</label>
	<div class="group relative transition-all">
		<div
			class="bg-mofu-dark-700 border-mofu-dark-600 relative aspect-video w-full overflow-hidden rounded-lg border group-hover:opacity-75"
		>
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
				<div
					class="bg-opacity-0 group-hover:bg-opacity-30 absolute inset-0 flex items-center justify-center bg-black transition-all duration-200"
				>
					<div class="flex gap-2 opacity-0 transition-opacity duration-200 group-hover:opacity-100">
						<label
							for="thumbnail-upload"
							class="cursor-pointer rounded bg-blue-600 px-3 py-1 text-sm text-white hover:bg-blue-700"
						>
							변경
						</label>
						<button onclick={removeThumbnail} class="rounded bg-red-600 px-3 py-1 text-sm text-white hover:bg-red-700">
							제거
						</button>
					</div>
				</div>
			{:else}
				<label
					for="thumbnail-upload"
					class="text-mofu-dark-300 hover:text-mofu-dark-200 flex h-full cursor-pointer flex-col items-center justify-center space-y-2 transition-colors"
				>
					<Icon src={Photo} class="h-12 w-12" />
					<span class="text-sm">썸네일 이미지 추가</span>
					<span class="text-mofu-dark-400 text-xs">16:9 비율 권장, 최대 3MB</span>
				</label>
			{/if}
		</div>
		<input id="thumbnail-upload" type="file" accept="image/*" class="hidden" onchange={handleImageChange} />
	</div>
	<p class="text-mofu-dark-400 mt-1 text-xs">
		썸네일은 포스트 목록과 공유 시 표시됩니다. 설정하지 않으면 기본 이미지가 사용됩니다.
	</p>
</div>

<ImageCropModal
	bind:isOpen={showCrop}
	imageSrc={tempImageSrc}
	aspectRatio={16 / 9}
	cropShape="rect"
	onCrop={handleCrop}
	onCancel={handleCropCancel}
/>

<script lang="ts">
	import { Icon, Camera } from 'svelte-hero-icons';
	import ImageCropModal from '../../modal/ImageCropModal.svelte';
	import { useImageCrop } from './useImageCrop';

	interface Props {
		profileImage: string | null;
		onUpdate: (data: { profileImageFile: Blob; profileImage: string }) => void;
	}

	let { profileImage, onUpdate }: Props = $props();

	let showCrop = $state(false);
	let tempImageSrc = $state('');

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

	async function handleCrop(data: {
		croppedAreaPixels: { x: number; y: number; width: number; height: number };
	}) {
		try {
			const { blob, url } = await cropImage(tempImageSrc, data, {
				maxFileSizeMB: 5,
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
</script>

<div class="space-y-4">
	<h2 class="text-2xl font-semibold">Profile Image</h2>
	<div class="flex items-center space-x-4">
		<div class="group relative transition-all">
			<div class="bg-mofu-dark-800 h-24 w-24 overflow-hidden rounded-full group-hover:opacity-75">
				{#if profileImage}
					<img src={profileImage} alt="Profile preview" class="h-full w-full object-cover" />
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
		<div class="text-mofu-dark-300 text-sm">
			<p>Recommended: 400x400px</p>
			<p>Max file size: 5MB</p>
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
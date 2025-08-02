<script lang="ts">
	import Cropper from 'svelte-easy-crop';

	let {
		isOpen = $bindable(false),
		imageSrc,
		aspectRatio = 1,
		cropShape = 'rect' as 'rect' | 'round',
		title = 'Crop Image',
		onCrop,
		onCancel
	} = $props<{
		isOpen: boolean;
		imageSrc: string;
		aspectRatio?: number;
		cropShape?: 'rect' | 'round';
		title?: string;
		onCrop: (data: { croppedAreaPixels: any; croppedAreaPercentage: any }) => void;
		onCancel: () => void;
	}>();

	let crop = $state({ x: 0, y: 0 });
	let zoom = $state(1);
	let croppedAreaPixels = $state<any>(null);
	let croppedAreaPercentage = $state<any>(null);

	function handleSave() {
		if (croppedAreaPixels && croppedAreaPercentage) {
			onCrop({ croppedAreaPixels, croppedAreaPercentage });
			isOpen = false;
		}
	}

	function handleCancel() {
		onCancel();
		isOpen = false;
	}
</script>

{#if isOpen}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/80">
		<div class="bg-mofu-dark-800 w-full max-w-2xl rounded-lg p-6">
			<h2 class="text-mofu-dark-200 mb-4 text-xl font-semibold">{title}</h2>

			<div class="relative h-96 w-full">
				<Cropper
					image={imageSrc}
					bind:crop
					bind:zoom
					aspect={aspectRatio}
					{cropShape}
					oncropcomplete={(e) => {
						// 최신 크롭 데이터만 조용히 저장
						croppedAreaPixels = e.pixels;
						croppedAreaPercentage = e.percent;
					}}
				/>
			</div>

			<div class="mt-6 flex justify-between">
				<button
					class="dark:bg-mofu-dark-700 text-mofu-dark-200 hover:bg-mofu-dark-600 rounded px-4 py-2 transition-colors"
					onclick={handleCancel}
				>
					Cancel
				</button>
				<button class="bg-mofu hover:bg-mofu/80 rounded px-4 py-2 text-white transition-colors" onclick={handleSave}>
					Crop & Save
				</button>
			</div>
		</div>
	</div>
{/if}

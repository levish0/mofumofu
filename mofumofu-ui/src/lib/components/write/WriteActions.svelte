<script lang="ts">
	import { Icon } from 'svelte-hero-icons';
	import { ArrowLeft, BookmarkSquare, PaperAirplane } from 'svelte-hero-icons';

	interface Props {
		onExit: () => void;
		onSaveDraft: () => void;
		onPublish: () => void;
	}

	const { onExit, onSaveDraft, onPublish }: Props = $props();
</script>

<div class="bg-mofu-dark-700 p-4">
	<div class="flex items-center justify-between">
		<button
			onclick={onExit}
			class="flex items-center gap-2 rounded px-4 py-2 text-gray-300 hover:bg-gray-700 hover:text-white"
		>
			<Icon src={ArrowLeft} class="h-6 w-6" />
			나가기
		</button>

		<div class="flex items-center gap-3">
			<button
				onclick={onSaveDraft}
				class="flex items-center gap-2 rounded border border-gray-600 bg-transparent px-4 py-2 text-gray-300 hover:bg-gray-700 hover:text-white"
			>
				<Icon src={BookmarkSquare} class="h-6 w-6" />
				임시저장
			</button>
			<button
				onclick={onPublish}
				class="flex items-center gap-2 rounded bg-teal-600 px-4 py-2 text-white hover:bg-teal-700"
			>
				<Icon src={PaperAirplane} class="h-6 w-6" />
				Publish
			</button>
		</div>
	</div>
</div>

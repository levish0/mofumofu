<script lang="ts">
	import { BookmarkSquare, PaperAirplane, Icon, ClipboardDocumentList } from 'svelte-hero-icons';
	import { Button } from '../ui/button';
	import { ArrowLeft, Save, Send } from '@lucide/svelte';

	interface Props {
		onExit: () => void;
		onSaveDraft: () => void;
		onPublish: () => void;
	}

	const { onExit, onSaveDraft, onPublish }: Props = $props();
</script>

<div class="bg-mofu-dark-950 p-4">
	<div class="flex items-center justify-between">
		<Button
			variant="ghost"
			onclick={onExit}
			class="dark:text-mofu-dark-200 flex items-center gap-2 rounded px-4 py-2 text-lg"
		>
			<ArrowLeft class="h-5 w-5" />
			돌아가기
		</Button>

		<div class="flex items-center gap-3">
			<Button
				variant="ghost"
				onclick={onSaveDraft}
				class=" dark:text-mofu-dark-200 flex items-center gap-2 rounded px-4 py-2 text-lg"
			>
				<Icon src={ClipboardDocumentList} class="h-5 w-5" solid />
				임시저장
			</Button>
			<Button
				variant="ghost"
				onclick={onPublish}
				class=" dark:text-mofu-dark-950 dark:hover:bg-mofu bg-mofu flex items-center gap-2 rounded px-4 py-2 text-lg "
			>
				<Icon src={PaperAirplane} class="h-5 w-5" solid />
				출간하기
			</Button>
		</div>
	</div>
</div>

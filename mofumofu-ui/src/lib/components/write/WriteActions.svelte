<script lang="ts">
	import { BookmarkSquare, PaperAirplane, Icon, ClipboardDocumentList } from 'svelte-hero-icons';
	import { Button } from '../ui/button';
	import { ArrowLeft, Save, Send } from '@lucide/svelte';
	import { PublishDialog } from './PublishDialog';

	interface Props {
		title: string;
		content: string;
		tags: string;
		onExit: () => void;
		onSaveDraft: () => void;
		onPublished: () => void;
		isEditMode?: boolean;
		editSlug?: string;
		editPostId?: string;
	}

	const {
		title,
		content,
		tags,
		onExit,
		onSaveDraft,
		onPublished,
		isEditMode = false,
		editSlug,
		editPostId
	}: Props = $props();
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

			<PublishDialog {title} {content} {tags} {onPublished} {isEditMode} {editSlug} {editPostId} />
		</div>
	</div>
</div>

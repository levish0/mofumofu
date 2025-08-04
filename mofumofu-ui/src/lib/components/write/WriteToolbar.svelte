<script lang="ts">
	import {
		Heading1,
		Heading2,
		Heading3,
		Heading4,
		Bold,
		Italic,
		Strikethrough,
		Quote,
		Link,
		Image,
		Code,
		Sigma,
		ChevronUp,
		ChevronDown
	} from '@lucide/svelte';
	import { Button } from '../ui/button';

	interface Props {
		onInsertText: (before: string, after?: string) => void;
		showStickyToolbar: boolean;
		onToggleHeader: () => void;
	}

	const { onInsertText, showStickyToolbar, onToggleHeader }: Props = $props();
</script>

<div class="px-4 pb-4">
	<div class="flex flex-wrap items-center justify-between gap-2">
		<div class="flex flex-wrap items-center gap-2">
			<Button
				variant="ghost"
				onclick={() => onInsertText('# ')}
				class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-300 hover:dark:text-mofu-dark-200 rounded p-2"
			>
				<Heading1 class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('## ')}
				class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-300 hover:dark:text-mofu-dark-200 rounded p-2"
			>
				<Heading2 class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('### ')}
				class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-300 hover:dark:text-mofu-dark-200 rounded p-2"
			>
				<Heading3 class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('#### ')}
				class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-300 hover:dark:text-mofu-dark-200 rounded p-2"
			>
				<Heading4 class="h-5 w-5" />
			</Button>

			<div class="bg-mofu-dark-700 mx-2 h-6 w-px"></div>

			<Button
				variant="ghost"
				onclick={() => onInsertText('**', '**')}
				class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-300 hover:dark:text-mofu-dark-200 rounded p-2 "
				title="굵게"
			>
				<Bold class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('*', '*')}
				class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-300 hover:dark:text-mofu-dark-200 rounded p-2"
				title="기울임"
			>
				<Italic class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('~~', '~~')}
				class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-300 hover:dark:text-mofu-dark-200 rounded p-2"
				title="취소선"
			>
				<Strikethrough class="h-5 w-5" />
			</Button>

			<div class="bg-mofu-dark-700 mx-2 h-6 w-px"></div>

			<Button
				variant="ghost"
				onclick={() => onInsertText('> ')}
				class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-300 hover:dark:text-mofu-dark-200 rounded p-2"
				title="인용"
			>
				<Quote class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('[', '](url)')}
				class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-300 hover:dark:text-mofu-dark-200 rounded p-2"
				title="링크"
			>
				<Link class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('![alt](', ')')}
				class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-300 hover:dark:text-mofu-dark-200 rounded p-2"
				title="이미지"
			>
				<Image class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('$$\n', '\n$$')}
				class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-300 hover:dark:text-mofu-dark-200 rounded p-2"
				title="수식"
			>
				<Sigma class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('```language', '\n```')}
				class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-300 hover:dark:text-mofu-dark-200 rounded p-2"
				title="코드"
			>
				<Code class="h-5 w-5" />
			</Button>
		</div>

		<!-- 헤더 토글 버튼 -->
		<Button
			variant="ghost"
			onclick={onToggleHeader}
			class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-300 hover:dark:text-mofu-dark-200 rounded p-2"
			title={showStickyToolbar ? '헤더 보이기' : '헤더 숨기기'}
		>
			{#if showStickyToolbar}
				<ChevronDown class="h-5 w-5" />
			{:else}
				<ChevronUp class="h-5 w-5" />
			{/if}
		</Button>
	</div>
</div>

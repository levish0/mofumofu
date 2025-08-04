<script lang="ts">
	import { Badge } from '../ui/badge';

	interface Props {
		title: string;
		tags: string;
		showStickyToolbar: boolean;
		onTitleChange: (value: string) => void;
		onTagsChange: (value: string) => void;
	}

	const { title, tags, showStickyToolbar, onTitleChange, onTagsChange }: Props = $props();

	const tagArray = $derived([
		...new Set(
			tags
				.split(/[,\n]/)
				.map((tag) => tag.trim())
				.filter((tag) => tag.length > 0)
		)
	]);

	let currentInput = $state('');

	function handleTagKeyPress(event: KeyboardEvent) {
		if (event.key === 'Enter' || event.key === ',') {
			event.preventDefault();
			addCurrentTag();
		} else if (event.key === 'Backspace' && currentInput === '' && tagArray.length > 0) {
			event.preventDefault();
			removeTag(tagArray[tagArray.length - 1]);
		}
	}

	function addCurrentTag() {
		const trimmedTag = currentInput.trim();
		if (trimmedTag) {
			const currentTags = new Set(
				tags
					.split(/[,\n]/)
					.map((tag) => tag.trim())
					.filter((tag) => tag.length > 0)
			);
			currentTags.add(trimmedTag);
			onTagsChange([...currentTags].join(','));
			currentInput = '';
		}
	}

	function removeTag(tagToRemove: string) {
		const currentTags = new Set(
			tags
				.split(/[,\n]/)
				.map((tag) => tag.trim())
				.filter((tag) => tag.length > 0)
		);
		currentTags.delete(tagToRemove);
		onTagsChange([...currentTags].join(','));
	}
</script>

<div
	class="overflow-hidden pt-4 break-all transition-all duration-400 ease-in-out"
	style="max-height: {showStickyToolbar ? '0px' : '480px'}; opacity: {showStickyToolbar ? '0' : '1'};"
>
	<!-- 제목 입력 -->
	<div class="mb-6 px-6">
		<input
			value={title}
			oninput={(e) => onTitleChange((e.target as HTMLInputElement).value)}
			placeholder="제목을 입력하세요"
			class="placeholder:text-mofu-dark-600 text-mofu-dark-200 h-auto w-full border-none bg-transparent p-0 text-4xl font-bold outline-none"
			style="font-size: 2.5rem; line-height: 1.2;"
		/>
	</div>

	<!-- 태그 입력 -->
	<div class="mb-4 px-6">
		<div class="flex flex-wrap items-center gap-2 text-lg">
			<!-- 기존 태그들 표시 -->
			{#each tagArray as tag}
				<Badge
					variant="default"
					class="hover:bg-mofu-dark-700 bg-mofu-dark-800 text-mofu cursor-pointer"
					onclick={() => removeTag(tag)}
				>
					#{tag}
				</Badge>
			{/each}

			<!-- 태그 입력 필드 -->
			<input
				bind:value={currentInput}
				onkeydown={handleTagKeyPress}
				placeholder="태그를 입력하세요"
				class="placeholder:text-mofu-dark-600 text-mofu-dark-200 min-w-[120px] flex-1 border-none bg-transparent p-0 outline-none"
			/>
		</div>
	</div>
</div>

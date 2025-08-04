<script lang="ts">
	interface Props {
		title: string;
		tags: string;
		showStickyToolbar: boolean;
		onTitleChange: (value: string) => void;
		onTagsChange: (value: string) => void;
	}

	const { title, tags, showStickyToolbar, onTitleChange, onTagsChange }: Props = $props();
</script>

<div
	class="overflow-hidden pt-6 transition-all duration-400 ease-in-out"
	style="max-height: {showStickyToolbar ? '0px' : '480px'}; opacity: {showStickyToolbar ? '0' : '1'};"
>
	<!-- 제목 입력 -->
	<div class="mb-6 px-6">
		<input
			value={title}
			oninput={(e) => onTitleChange(e.target.value)}
			placeholder="제목을 입력하세요"
			class="placeholder:text-mofu-dark-300 h-auto w-full border-none bg-transparent p-0 text-4xl font-bold text-white outline-none"
			style="font-size: 2.5rem; line-height: 1.2;"
		/>
	</div>

	<!-- 태그 입력 -->
	<div class="mb-8 px-6">
		<input
			value={tags}
			oninput={(e) => onTagsChange(e.target.value)}
			placeholder="태그를 입력하세요"
			class="h-auto w-full border-none bg-transparent p-0 text-lg text-gray-300 outline-none placeholder:text-gray-500"
		/>
	</div>
</div>

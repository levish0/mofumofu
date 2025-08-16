<script lang="ts">
	import '$lib/styles/markdown.css';
	import { useResizable } from '$lib/hooks/ui/useResizable.svelte';
	import WriteEditor from '$lib/components/write/WriteEditor.svelte';
	import WritePreview from '$lib/components/write/WritePreview.svelte';
	import { processMarkdown } from '$lib/utils/markdown';
	import * as m from '../../paraglide/messages';

	let title = $state('');
	let tags = $state('');
	let content = $state('');
	let htmlOutput = $state('');
	let containerElement: HTMLElement;

	// Resizable hook
	let resizableHook = $state<ReturnType<typeof useResizable> | null>(null);

	$effect(() => {
		if (containerElement) {
			resizableHook = useResizable(containerElement);
		}
	});

	$effect(() => {
		(async () => {
			const { htmlContent } = await processMarkdown(content);
			htmlOutput = htmlContent;
		})();
	});

	function handleTitleChange(value: string) {
		title = value;
	}

	function handleTagsChange(value: string) {
		tags = value;
	}

	function handleContentChange(value: string) {
		content = value;
	}

	function handleSaveDraft() {
		console.log('임시저장:', { title, tags, content });
	}

	function handleExit() {
		history.back();
	}
</script>

<svelte:head>
	<title>{m.write_page_title()}</title>
	<meta name="description" content={m.write_page_description()} />
	<meta name="robots" content="noindex, nofollow" />

	<!-- Open Graph -->
	<meta property="og:title" content={m.write_page_title()} />
	<meta property="og:description" content={m.write_page_description()} />
	<meta property="og:type" content="website" />
	<meta property="og:site_name" content="Mofu" />

	<!-- Twitter Card -->
	<meta name="twitter:card" content="summary" />
	<meta name="twitter:title" content={m.write_page_title()} />
	<meta name="twitter:description" content={m.write_page_description()} />
</svelte:head>

<div class="bg-mofu-dark-900 flex h-full w-full break-all text-white">
	<!-- 메인 컨텐츠 영역 -->
	<div bind:this={containerElement} class="flex flex-1 overflow-hidden">
		<!-- 에디터 영역 -->
		<div style="width: {resizableHook?.leftWidth() ?? 50}%">
			<WriteEditor
				{title}
				{tags}
				{content}
				onTitleChange={handleTitleChange}
				onTagsChange={handleTagsChange}
				onContentChange={handleContentChange}
				onExit={handleExit}
				onSaveDraft={handleSaveDraft}
				onPublished={() => {}}
			/>
		</div>

		<!-- Resizer (드래그 핸들) -->
		<button
			type="button"
			aria-label={m.write_resize_handle()}
			class="bg-mofu-dark-700 w-1 flex-shrink-0 cursor-col-resize p-0 transition-colors"
			onmousedown={resizableHook?.handleMouseDown}
			class:bg-gray-400={resizableHook?.isDragging()}
		></button>

		<!-- 미리보기 영역 -->
		<div style="width: {resizableHook?.rightWidth() ?? 50}%">
			<WritePreview {title} {htmlOutput} />
		</div>
	</div>
</div>

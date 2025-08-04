<script lang="ts">
	import '$lib/styles/markdown.css';
	import { useResizable } from '$lib/hooks/useResizable.svelte';
	import WriteEditor from '$lib/components/write/WriteEditor.svelte';
	import WritePreview from '$lib/components/write/WritePreview.svelte';

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

	async function processMarkdown(markdown: string) {
		try {
			const { unified } = await import('unified');
			const { default: remarkParse } = await import('remark-parse');
			const { default: remarkGfm } = await import('remark-gfm');
			const { default: remarkBreaks } = await import('remark-breaks');
			const { default: remarkToc } = await import('remark-toc');
			const { default: remarkMath } = await import('remark-math');
			const { default: remarkEmoji } = await import('remark-emoji');
			const { default: remarkGithubBlockquoteAlert } = await import('remark-github-blockquote-alert');
			const { default: remarkRehype } = await import('remark-rehype');
			const { default: rehypeKatex } = await import('rehype-katex');
			const { default: rehypeHighlight } = await import('rehype-highlight');
			const { default: rehypeAutolinkHeadings } = await import('rehype-autolink-headings');
			const { default: rehypeSanitize, defaultSchema } = await import('rehype-sanitize');
			const { default: rehypeStringify } = await import('rehype-stringify');

			// Sanitize schema for GFM + KaTeX + Code highlighting
			const sanitizeSchema = {
				...defaultSchema,
				tagNames: [
					...(defaultSchema.tagNames || []),
					// GFM extras
					'input',
					'details',
					'summary',
					'del',
					'ins',
					// KaTeX
					'math',
					'semantics',
					'mrow',
					'mi',
					'mo',
					'mn',
					'msup',
					'msub',
					'mfrac',
					'munder',
					'mover',
					'munderover',
					'mtable',
					'mtr',
					'mtd',
					'mspace',
					'mtext',
					'annotation'
				],
				attributes: {
					...(defaultSchema.attributes || {}),
					'*': ['className', 'id'],
					// KaTeX classes
					span: ['className', 'style'],
					div: ['className', 'style'],
					// GFM checkboxes
					input: ['type', 'disabled', 'checked'],
					// Code highlighting
					pre: ['className', 'style'],
					code: ['className', 'style'],
					// Links
					a: ['href', 'title', 'target', 'rel']
				},
				protocols: {
					...(defaultSchema.protocols || {}),
					href: ['http', 'https', 'mailto']
				}
			};

			const result = await unified()
				.use(remarkParse)
				.use(remarkGfm)
				.use(remarkBreaks)
				.use(remarkToc)
				.use(remarkMath)
				.use(remarkEmoji)
				.use(remarkGithubBlockquoteAlert)
				.use(remarkRehype, { allowDangerousHtml: true })
				.use(rehypeKatex)
				.use(rehypeHighlight)
				.use(rehypeAutolinkHeadings)
				.use(rehypeSanitize, sanitizeSchema)
				.use(rehypeStringify, { allowDangerousHtml: true })
				.process(markdown);

			htmlOutput = String(result);
		} catch (error) {
			console.error('Markdown processing error:', error);
			htmlOutput = '<p>마크다운 처리 중 오류가 발생했습니다.</p>';
		}
	}

	$effect(() => {
		processMarkdown(content);
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

	function handlePublish() {
		console.log('출간:', { title, tags, content });
	}

	function handleExit() {
		history.back();
	}
</script>

<svelte:head>
	<title>글 작성 - mofu</title>
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
				onPublish={handlePublish}
			/>
		</div>

		<!-- Resizer (드래그 핸들) -->
		<button
			type="button"
			aria-label="크기 조정"
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

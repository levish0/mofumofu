<script lang="ts">
	import '$lib/styles/markdown.css';
	import { unified } from 'unified';
	import remarkParse from 'remark-parse';
	import remarkGfm from 'remark-gfm';
	import remarkToc from 'remark-toc';
	import remarkMath from 'remark-math';
	import remarkEmoji from 'remark-emoji';
	import remarkGithubBlockquoteAlert from 'remark-github-blockquote-alert';
	import remarkRehype from 'remark-rehype';
	import rehypeKatex from 'rehype-katex';
	import rehypeHighlight from 'rehype-highlight';
	import rehypeAutolinkHeadings from 'rehype-autolink-headings';
	import rehypeStringify from 'rehype-stringify';
	import { Icon } from 'svelte-hero-icons';
	import {
		Bold,
		Italic,
		Strikethrough,
		ChatBubbleLeft,
		Link,
		Photo,
		CodeBracket,
		ArrowLeft,
		BookmarkSquare,
		PaperAirplane
	} from 'svelte-hero-icons';
	import { useTextareaToolbar } from '$lib/hooks/useTextareaToolbar.svelte';

	let title = $state('');
	let tags = $state('');
	let content = $state('');
	let htmlOutput = $state('');
	let contentTextarea: HTMLTextAreaElement;

	// Sticky toolbar hook - reactive to textarea binding
	let toolbarHook = $state<ReturnType<typeof useTextareaToolbar> | null>(null);

	$effect(() => {
		if (contentTextarea) {
			toolbarHook = useTextareaToolbar(contentTextarea);
		}
	});

	async function processMarkdown(markdown: string) {
		try {
			const result = await unified()
				.use(remarkParse)
				.use(remarkGfm)
				.use(remarkToc)
				.use(remarkMath)
				.use(remarkEmoji)
				.use(remarkGithubBlockquoteAlert)
				.use(remarkRehype, { allowDangerousHtml: true })
				.use(rehypeKatex)
				.use(rehypeHighlight)
				.use(rehypeAutolinkHeadings)
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

	function insertText(before: string, after: string = '') {
		if (!contentTextarea) return;

		const start = contentTextarea.selectionStart;
		const end = contentTextarea.selectionEnd;
		const selectedText = content.substring(start, end);
		const newText = content.substring(0, start) + before + selectedText + after + content.substring(end);

		content = newText;

		setTimeout(() => {
			contentTextarea.focus();
			contentTextarea.setSelectionRange(start + before.length, start + before.length + selectedText.length);
		}, 0);
	}

	function handleSaveDraft() {
		console.log('임시저장:', { title, tags, content });
	}

	function handlePublish() {
		console.log('출간:', { title, tags, content });
	}

	function handleExit() {
		console.log('나가기');
	}
</script>

<svelte:head>
	<title>글 작성 - mofu</title>
</svelte:head>

<div class="bg-mofu-dark-900 flex h-full w-full break-all text-white">
	<!-- 메인 컨텐츠 영역 -->
	<div class="flex flex-1 overflow-hidden">
		<!-- 에디터 영역 (왼쪽 절반) -->
		<div class="bg-mofu-dark-900 flex h-full w-1/2 flex-col">
			<!-- 헤더 영역 (sticky) -->
			<div class="bg-mofu-dark-900 sticky top-0 z-10 overflow-hidden">
				<!-- 제목/태그 영역 -->
				<div
					class="overflow-hidden pt-4 transition-all duration-500 ease-out"
					style="max-height: {toolbarHook?.showStickyToolbar()
						? '0px'
						: '480px'}; opacity: {toolbarHook?.showStickyToolbar() ? '0' : '1'};"
				>
					<!-- 제목 입력 -->
					<div class="mb-6 px-4">
						<input
							bind:value={title}
							placeholder="제목을 입력하세요"
							class="placeholder:text-mofu-dark-300 h-auto w-full border-none bg-transparent p-0 text-4xl font-bold text-white outline-none"
							style="font-size: 2.5rem; line-height: 1.2;"
						/>
					</div>

					<!-- 구분선 -->
					<div class="mb-8 h-1 w-16 bg-gray-600 px-4"></div>

					<!-- 태그 입력 -->
					<div class="mb-8 px-4">
						<input
							bind:value={tags}
							placeholder="태그를 입력하세요"
							class="h-auto w-full border-none bg-transparent p-0 text-lg text-gray-300 outline-none placeholder:text-gray-500"
						/>
					</div>
				</div>

				<!-- 툴바 (항상 보임) -->
				<div class="px-4 pb-4">
					<div class="flex flex-wrap items-center gap-2">
						<button
							onclick={() => insertText('# ')}
							class="rounded px-3 py-2 text-gray-300 hover:bg-gray-700 hover:text-white"
						>
							H1
						</button>
						<button
							onclick={() => insertText('## ')}
							class="rounded px-3 py-2 text-gray-300 hover:bg-gray-700 hover:text-white"
						>
							H2
						</button>
						<button
							onclick={() => insertText('### ')}
							class="rounded px-3 py-2 text-gray-300 hover:bg-gray-700 hover:text-white"
						>
							H3
						</button>
						<button
							onclick={() => insertText('#### ')}
							class="rounded px-3 py-2 text-gray-300 hover:bg-gray-700 hover:text-white"
						>
							H4
						</button>

						<div class="mx-2 h-6 w-px bg-gray-600"></div>

						<button
							onclick={() => insertText('**', '**')}
							class="rounded p-2 text-gray-300 hover:bg-gray-700 hover:text-white"
							title="굵게"
						>
							<Icon src={Bold} solid class="h-4 w-4" />
						</button>
						<button
							onclick={() => insertText('*', '*')}
							class="rounded p-2 text-gray-300 hover:bg-gray-700 hover:text-white"
							title="기울임"
						>
							<Icon src={Italic} solid class="h-4 w-4" />
						</button>
						<button
							onclick={() => insertText('~~', '~~')}
							class="rounded p-2 text-gray-300 hover:bg-gray-700 hover:text-white"
							title="취소선"
						>
							<Icon src={Strikethrough} solid class="h-4 w-4" />
						</button>

						<div class="mx-2 h-6 w-px bg-gray-600"></div>

						<button
							onclick={() => insertText('> ')}
							class="rounded p-2 text-gray-300 hover:bg-gray-700 hover:text-white"
							title="인용"
						>
							<Icon src={ChatBubbleLeft} solid class="h-4 w-4" />
						</button>
						<button
							onclick={() => insertText('[', '](url)')}
							class="rounded p-2 text-gray-300 hover:bg-gray-700 hover:text-white"
							title="링크"
						>
							<Icon src={Link} solid class="h-4 w-4" />
						</button>
						<button
							onclick={() => insertText('![alt](', ')')}
							class="rounded p-2 text-gray-300 hover:bg-gray-700 hover:text-white"
							title="이미지"
						>
							<Icon src={Photo} solid class="h-4 w-4" />
						</button>
						<button
							onclick={() => insertText('`', '`')}
							class="rounded p-2 text-gray-300 hover:bg-gray-700 hover:text-white"
							title="코드"
						>
							<Icon src={CodeBracket} class="h-4 w-4" />
						</button>
					</div>
				</div>
			</div>

			<!-- 본문 입력 -->
			<div class="flex flex-1 flex-col">
				<textarea
					bind:this={contentTextarea}
					bind:value={content}
					placeholder="당신의 이야기를 적어보세요..."
					class="w-full flex-1 resize-none border-none bg-transparent px-4 py-0 text-lg leading-relaxed text-white outline-none placeholder:text-gray-500"
					spellcheck="false"
				></textarea>
			</div>
			<!-- 에디터 영역 하단 버튼들 -->
			<div class=" bg-mofu-dark-700 p-4">
				<div class="flex items-center justify-between">
					<button
						onclick={handleExit}
						class="flex items-center gap-2 rounded px-4 py-2 text-gray-300 hover:bg-gray-700 hover:text-white"
					>
						<Icon src={ArrowLeft} class="h-4 w-4" />
						나가기
					</button>

					<div class="flex items-center gap-3">
						<button
							onclick={handleSaveDraft}
							class="flex items-center gap-2 rounded border border-gray-600 bg-transparent px-4 py-2 text-gray-300 hover:bg-gray-700 hover:text-white"
						>
							<Icon src={BookmarkSquare} class="h-4 w-4" />
							임시저장
						</button>
						<button
							onclick={handlePublish}
							class="flex items-center gap-2 rounded bg-teal-600 px-4 py-2 text-white hover:bg-teal-700"
						>
							<Icon src={PaperAirplane} class="h-4 w-4" />
							Publish
						</button>
					</div>
				</div>
			</div>
		</div>

		<!-- 미리보기 영역 -->
		<div class="bg-mofu-dark-950 flex w-1/2 flex-col pt-4 pl-8">
			<div class="mb-6">
				<h1 class="text-4xl font-bold text-white" style="font-size: 2.5rem; line-height: 1.2;">
					{title || '제목을 입력하세요'}
				</h1>
			</div>

			<div class="mb-8 h-1 w-16 bg-gray-600"></div>

			{#if tags}
				<div class="mb-8">
					<div class="flex flex-wrap gap-2">
						{#each tags.split(',') as tag}
							<span class="rounded bg-gray-700 px-2 py-1 text-sm text-gray-300">
								{tag.trim()}
							</span>
						{/each}
					</div>
				</div>
			{/if}

			<div class="flex-1 overflow-auto pr-4 pb-4">
				<div class="prose prose-invert prose-lg max-w-none text-white">
					{@html htmlOutput}
				</div>
			</div>
		</div>
	</div>
</div>

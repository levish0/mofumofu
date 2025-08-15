<script lang="ts">
	import '$lib/styles/markdown.css';
	import { useResizable } from '$lib/hooks/ui/useResizable.svelte';
	import WriteEditor from '$lib/components/write/WriteEditor.svelte';
	import WritePreview from '$lib/components/write/WritePreview.svelte';
	import { processMarkdown } from '$lib/utils/markdown';
	import { updatePost } from '$lib/api/post/postApi';
	import { userStore } from '$lib/stores/user.svelte';
	import { goto } from '$app/navigation';
	import { toast } from 'svelte-sonner';
	import type { PageData } from './$types';
	import * as m from '../../../paraglide/messages';

	const { data }: { data: PageData } = $props();

	// 현재 사용자 확인 (권한 체크는 서버에서 하지만 클라이언트에서도 확인)
	const currentUser = $derived(userStore.user);

	// 기존 포스트 데이터로 초기화
	let title = $state(data.post.title);
	let tags = $state(data.post.tags.join(', '));
	let content = $state(data.post.content); // 이제 원본 마크다운을 가져올 수 있음
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

	async function handlePublished() {
		try {
			const hashtags = tags
				.split(',')
				.map((tag) => tag.trim())
				.filter((tag) => tag.length > 0);

			await updatePost({
				slug: data.slug,
				title: title || null,
				content: content || null,
				hashtags: hashtags.length > 0 ? hashtags : null,
				summary: data.post.summary || null
			});

			toast.success(m.edit_success_message());

			// 수정 완료 후 포스트 페이지로 이동 - 현재 사용자 핸들 필요
			if (currentUser) {
				goto(`/@${currentUser.handle}/post/${data.slug}`);
			}
		} catch (error) {
			console.error('Failed to update post:', error);
			toast.error(m.edit_error_message());
		}
	}

	function handleExit() {
		// 뒤로 가기 또는 홈으로
		if (currentUser) {
			goto(`/@${currentUser.handle}/profile`);
		} else {
			goto('/');
		}
	}
</script>

<svelte:head>
	<title>{m.edit_page_title({ title: data.post.title })}</title>
	<meta name="description" content={m.edit_page_description()} />
	<meta name="robots" content="noindex, nofollow" />

	<!-- Open Graph -->
	<meta property="og:title" content="글 수정 - Mofumofu" />
	<meta property="og:description" content="포스트를 수정합니다." />
	<meta property="og:type" content="website" />
	<meta property="og:site_name" content="Mofumofu" />

	<!-- Twitter Card -->
	<meta name="twitter:card" content="summary" />
	<meta name="twitter:title" content="글 수정 - Mofumofu" />
	<meta name="twitter:description" content="포스트를 수정합니다." />
</svelte:head>

<div class="flex h-full w-full bg-gray-900 break-all text-white dark:bg-gray-900">
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
				onPublished={handlePublished}
				isEditMode={true}
			/>
		</div>

		<!-- Resizer (드래그 핸들) -->
		<button
			type="button"
			aria-label="크기 조정"
			class="w-1 flex-shrink-0 cursor-col-resize bg-gray-700 p-0 transition-colors"
			onmousedown={resizableHook?.handleMouseDown}
			class:bg-gray-400={resizableHook?.isDragging()}
		></button>

		<!-- 미리보기 영역 -->
		<div style="width: {resizableHook?.rightWidth() ?? 50}%">
			<WritePreview {title} {htmlOutput} />
		</div>
	</div>
</div>

<script lang="ts">
	import { onMount } from 'svelte';
	import { useComments } from '$lib/hooks/comment/useComments.svelte';
	import Comment from './Comment.svelte';
	import CommentForm from './CommentForm.svelte';
	import type { CommentInfo } from '$lib/api/comment/types';

	interface Props {
		postId: string;
	}

	let { postId }: Props = $props();

	const comments = useComments(postId, 5);

	onMount(() => {
		comments.loadComments();
	});

	const handleNewComment = (newComment: CommentInfo) => {
		comments.addComment(newComment);
	};

	const handleReply = (newReply: CommentInfo) => {
		// 답글이 달린 경우 전체 댓글을 새로고침
		// (실제로는 더 효율적인 방법을 사용할 수 있음)
		setTimeout(() => {
			comments.refreshComments();
		}, 500);
	};
</script>

<div class="comments-section">
	<!-- 댓글 섹션 헤더 -->
	<div class="mb-6 flex items-center justify-between">
		<h3 class="text-lg font-semibold text-black dark:text-white">
			댓글 {comments.totalCount > 0 ? `(${comments.totalCount})` : ''}
		</h3>
	</div>

	<!-- 댓글 작성 폼 -->
	<div class="mb-8">
		<CommentForm {postId} onSubmit={handleNewComment} />
	</div>

	<!-- 댓글 목록 -->
	<div class="comments-list">
		{#if comments.loading && comments.comments.length === 0}
			<!-- 로딩 스켈레톤 -->
			<div class="space-y-4">
				{#each Array(3) as _}
					<div class="comment-skeleton animate-pulse">
						<div class="flex gap-3">
							<div class="flex-shrink-0">
								<div class="bg-mofu-light-600 dark:bg-mofu-dark-600 h-10 w-10 rounded-full"></div>
							</div>
							<div class="flex-1 space-y-2">
								<div class="flex items-center gap-2">
									<div class="bg-mofu-light-600 dark:bg-mofu-dark-600 h-3 w-20 rounded"></div>
									<div class="bg-mofu-light-600 dark:bg-mofu-dark-600 h-3 w-16 rounded"></div>
									<div class="bg-mofu-light-600 dark:bg-mofu-dark-600 h-3 w-12 rounded"></div>
								</div>
								<div class="space-y-1">
									<div class="bg-mofu-light-600 dark:bg-mofu-dark-600 h-4 w-full rounded"></div>
									<div class="bg-mofu-light-600 dark:bg-mofu-dark-600 h-4 w-3/4 rounded"></div>
								</div>
								<div class="flex items-center gap-4">
									<div class="bg-mofu-light-600 dark:bg-mofu-dark-600 h-3 w-8 rounded"></div>
									<div class="bg-mofu-light-600 dark:bg-mofu-dark-600 h-3 w-8 rounded"></div>
									<div class="bg-mofu-light-600 dark:bg-mofu-dark-600 h-3 w-12 rounded"></div>
								</div>
							</div>
						</div>
					</div>
				{/each}
			</div>
		{:else if comments.comments.length === 0}
			<!-- 댓글 없음 -->
			<div class="text-mofu-light-400 dark:text-mofu-dark-400 py-12 text-center">
				<svg
					class="text-mofu-light-500 dark:text-mofu-dark-500 mx-auto mb-4 h-12 w-12"
					fill="none"
					stroke="currentColor"
					viewBox="0 0 24 24"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"
					/>
				</svg>
				<p class="text-sm">첫 번째 댓글을 작성해보세요!</p>
			</div>
		{:else}
			<!-- 댓글 리스트 -->
			<div class="space-y-4">
				{#each comments.comments as comment}
					<Comment {comment} {postId} onReply={handleReply} />
				{/each}
			</div>

			<!-- 더 많은 댓글 로드 버튼 -->
			{#if comments.hasMore}
				<div class="mt-6 text-center">
					<button
						onclick={comments.loadMoreComments}
						disabled={comments.loading}
						class="text-mofu-light-200 dark:text-mofu-dark-200 bg-mofu-light-800 dark:bg-mofu-dark-800 hover:bg-mofu-light-700 dark:hover:bg-mofu-dark-700 disabled:bg-mofu-light-600 dark:disabled:bg-mofu-dark-600 disabled:text-mofu-light-400 dark:disabled:text-mofu-dark-400 inline-flex items-center rounded-lg border border-gray-200 px-6 py-2 text-sm font-medium transition-colors focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed dark:border-gray-700"
					>
						{#if comments.loading}
							<svg
								class="text-mofu-light-400 dark:text-mofu-dark-400 mr-2 -ml-1 h-4 w-4 animate-spin"
								fill="none"
								viewBox="0 0 24 24"
							>
								<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
								<path
									class="opacity-75"
									fill="currentColor"
									d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
								></path>
							</svg>
							로딩 중...
						{:else}
							댓글 더 보기
						{/if}
					</button>
				</div>
			{/if}

			<!-- 현재 로딩 중인 경우 하단 로딩 표시 -->
			{#if comments.loading && comments.comments.length > 0}
				<div class="mt-4 text-center">
					<div class="inline-flex items-center text-sm text-gray-500">
						<svg class="mr-2 -ml-1 h-4 w-4 animate-spin text-gray-400" fill="none" viewBox="0 0 24 24">
							<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
							<path
								class="opacity-75"
								fill="currentColor"
								d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
							></path>
						</svg>
						댓글을 불러오는 중...
					</div>
				</div>
			{/if}
		{/if}
	</div>
</div>

<style>
	.comment-skeleton {
		padding: 1rem 0;
		border-bottom: 1px solid #f3f4f6;
	}
</style>

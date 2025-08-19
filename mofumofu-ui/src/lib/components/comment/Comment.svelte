<script lang="ts">
	import type { CommentInfo } from '$lib/api/comment/types';
	import { useReplies } from '$lib/hooks/comment/useReplies.svelte';
	import { createCommentLike, deleteCommentLike } from '$lib/api/comment/commentApi';
	import { authStore } from '$lib/stores/auth.svelte';
	import { userStore } from '$lib/stores/user.svelte';
	import CommentForm from './CommentForm.svelte';
	import Self from './Comment.svelte';

	interface Props {
		comment: CommentInfo;
		postId: string;
		onReply?: (newComment: CommentInfo) => void;
		isReply?: boolean;
	}

	let { comment, postId, onReply, isReply = false }: Props = $props();

	const replies = useReplies(comment.id);

	let showReplyForm = $state(false);
	let isLiking = $state(false);
	let activeReplyId = $state<string | null>(null);

	// 삭제된 댓글인지 확인
	const isDeleted = $derived(comment.is_deleted);
	const displayContent = $derived(isDeleted ? '[삭제된 댓글입니다]' : comment.content);
	const displayUserName = $derived(isDeleted ? '익명' : comment.user_name);
	const displayUserHandle = $derived(isDeleted ? '' : comment.user_handle);
	const displayUserImage = $derived(isDeleted ? null : comment.user_profile_image);

	// 상대 시간 표시
	const getRelativeTime = (dateString: string) => {
		const date = new Date(dateString);
		const now = new Date();
		const diffInMinutes = Math.floor((now.getTime() - date.getTime()) / (1000 * 60));

		if (diffInMinutes < 1) return '방금 전';
		if (diffInMinutes < 60) return `${diffInMinutes}분 전`;

		const diffInHours = Math.floor(diffInMinutes / 60);
		if (diffInHours < 24) return `${diffInHours}시간 전`;

		const diffInDays = Math.floor(diffInHours / 24);
		if (diffInDays < 7) return `${diffInDays}일 전`;

		return date.toLocaleDateString();
	};

	const toggleLike = async () => {
		if (!userStore.user || isLiking) return;

		try {
			isLiking = true;
			if (comment.is_liked) {
				await deleteCommentLike({ comment_id: comment.id });
				comment.is_liked = false;
				comment.like_count--;
			} else {
				await createCommentLike({ comment_id: comment.id });
				comment.is_liked = true;
				comment.like_count++;
			}
		} catch (error) {
			console.error('Failed to toggle comment like:', error);
		} finally {
			isLiking = false;
		}
	};

	const handleReplySubmit = (newComment: CommentInfo) => {
		replies.addReply(newComment);
		showReplyForm = false;
		activeReplyId = null;
		if (onReply) {
			onReply(newComment);
		}
	};

	const toggleReplyForm = (replyId?: string) => {
		if (replyId) {
			// 답글에 답글 달기
			if (activeReplyId === replyId) {
				activeReplyId = null;
			} else {
				activeReplyId = replyId;
				showReplyForm = false; // 메인 답글 폼은 닫기
			}
		} else {
			// 메인 댓글에 답글 달기
			showReplyForm = !showReplyForm;
			activeReplyId = null; // 답글의 답글 폼은 닫기
		}
	};

	const toggleReplyLike = async (reply: CommentInfo) => {
		if (!userStore.user) return;

		try {
			if (reply.is_liked) {
				await deleteCommentLike({ comment_id: reply.id });
				reply.is_liked = false;
				reply.like_count--;
			} else {
				await createCommentLike({ comment_id: reply.id });
				reply.is_liked = true;
				reply.like_count++;
			}
		} catch (error) {
			console.error('Failed to toggle reply like:', error);
		}
	};
</script>

<div class="relative {isReply ? 'ml-6 pl-6' : ''}">
	<!-- 레딧 스타일 트리 라인들 (답글일 때만) -->
	{#if isReply}
		<div
			class="border-mofu-light-300 dark:border-mofu-dark-500 absolute top-0 -left-6 h-6 w-6 rounded-bl-lg border-b-2 border-l-2"
		></div>
		<!-- 부모에서 오는 세로선 -->
		<div class="bg-mofu-light-300 dark:bg-mofu-dark-500 absolute -top-2 -left-6 h-8 w-0.5"></div>
	{/if}

	<!-- 댓글 메인 -->
	<div
		class="group/comment hover:bg-mofu-light-800 dark:hover:bg-mofu-dark-800 -m-2 flex cursor-pointer items-start gap-3 rounded-lg p-2 py-2 transition-colors"
	>
		<!-- 프로필 이미지 -->
		<div class="relative flex-shrink-0">
			{#if displayUserImage}
				<img
					src={displayUserImage}
					alt={displayUserName}
					class="bg-mofu-light-800 dark:bg-mofu-dark-800 relative z-10 h-6 w-6 rounded-full object-cover"
				/>
			{:else}
				<div
					class="bg-mofu-light-600 dark:bg-mofu-dark-600 relative z-10 flex h-6 w-6 items-center justify-center rounded-full"
				>
					<span class="text-mofu-light-200 dark:text-mofu-dark-200 text-xs font-medium">
						{displayUserName?.charAt(0)?.toUpperCase() || '?'}
					</span>
				</div>
			{/if}
		</div>

		<!-- 댓글 내용 -->
		<div class="min-w-0 flex-1">
			<!-- 상단: 사용자 정보와 액션 버튼들 -->
			<div class="mb-1 flex items-start justify-between">
				<div class="flex items-center gap-2">
					<span class="text-sm font-medium text-black dark:text-white">{displayUserName}</span>
					{#if displayUserHandle}
						<span class="text-mofu-light-400 dark:text-mofu-dark-400 text-xs">@{displayUserHandle}</span>
					{/if}
					<span class="text-mofu-light-400 dark:text-mofu-dark-400 text-xs">{getRelativeTime(comment.created_at)}</span>
				</div>

				<!-- 우측 액션 버튼들 -->
				{#if !isDeleted}
					<div class="flex items-center gap-2">
						<!-- 좋아요 버튼 -->
						<button
							onclick={toggleLike}
							disabled={!userStore.user || isLiking}
							class="flex items-center gap-1 rounded px-2 py-1 text-xs transition-colors hover:text-red-500"
							class:text-red-500={comment.is_liked}
							class:text-mofu-light-400={!comment.is_liked}
							class:dark:text-mofu-dark-400={!comment.is_liked}
						>
							<svg
								class="h-4 w-4"
								fill={comment.is_liked ? 'currentColor' : 'none'}
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"
								/>
							</svg>
							{comment.like_count > 0 ? comment.like_count : ''}
						</button>

						<!-- 드롭다운 메뉴 -->
						<button
							class="text-mofu-light-400 dark:text-mofu-dark-400 hover:text-mofu-light-200 dark:hover:text-mofu-dark-200 p-1 transition-colors"
							aria-label="댓글 옵션"
						>
							<svg class="h-4 w-4" fill="currentColor" viewBox="0 0 24 24">
								<path
									d="M12 8c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm0 2c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z"
								/>
							</svg>
						</button>
					</div>
				{/if}
			</div>

			<!-- 댓글 내용 -->
			<div
				class="text-mofu-light-200 dark:text-mofu-dark-200 mb-2 text-sm"
				class:text-mofu-light-400={isDeleted}
				class:dark:text-mofu-dark-400={isDeleted}
				class:italic={isDeleted}
			>
				{displayContent}
			</div>

			<!-- 하단 액션 버튼들 -->
			{#if !isDeleted}
				<div class="text-mofu-light-400 dark:text-mofu-dark-400 flex items-center gap-4 text-xs">
					<!-- 답글 버튼 -->
					{#if userStore.user}
						<button onclick={() => toggleReplyForm()} class="transition-colors hover:text-blue-500"> 답글 </button>
					{/if}
				</div>
			{/if}

			<!-- 답글 작성 폼 -->
			{#if showReplyForm && userStore.user}
				<div class="mt-3">
					<CommentForm
						{postId}
						parentId={comment.id}
						placeholder="답글을 작성하세요..."
						onSubmit={handleReplySubmit}
						onCancel={() => (showReplyForm = false)}
					/>
				</div>
			{/if}

			<!-- 답글에 대한 세로선과 버튼 (레딧 스타일) -->
			{#if comment.reply_count > 0}
				<div class="relative mt-2">
					<!-- 세로선 -->
					<div class="bg-mofu-light-300 dark:bg-mofu-dark-500 absolute top-0 left-3 h-full w-0.5"></div>
					<!-- +/- 버튼 -->
					<button
						class="bg-mofu-light-800 dark:bg-mofu-dark-800 hover:bg-mofu-light-700 dark:hover:bg-mofu-dark-700 border-mofu-light-300 dark:border-mofu-dark-500 absolute top-2 left-1 z-10 rounded-full border p-1"
						onclick={replies.toggleReplies}
						aria-label={replies.expanded ? '답글 접기' : '답글 펼치기'}
					>
						{#if replies.expanded}
							<svg
								class="text-mofu-light-400 dark:text-mofu-dark-400 h-3 w-3"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M18 12H6" />
							</svg>
						{:else}
							<svg
								class="text-mofu-light-400 dark:text-mofu-dark-400 h-3 w-3"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
							</svg>
						{/if}
					</button>
				</div>
			{/if}

			<!-- 답글 목록 -->
			{#if comment.reply_count > 0 && replies.expanded}
				{#each replies.replies as reply, i}
					<Self comment={reply} {postId} {onReply} isReply={true} />
				{/each}

				<!-- 더 많은 답글 로드 버튼 -->
				{#if replies.hasMore}
					<div class="relative py-2 pl-6">
						<div
							class="border-mofu-light-300 dark:border-mofu-dark-500 absolute top-0 -left-6 h-4 w-6 rounded-bl-lg border-b-2 border-l-2"
						></div>
						<div class="bg-mofu-light-300 dark:bg-mofu-dark-500 absolute -top-2 -left-6 h-6 w-0.5"></div>
						<button
							onclick={replies.loadMoreReplies}
							disabled={replies.loading}
							class="text-mofu-light-400 dark:text-mofu-dark-400 hover:text-mofu-light-200 dark:hover:text-mofu-dark-200 flex cursor-pointer items-center gap-1.5 text-sm hover:underline"
						>
							{#if replies.loading}
								<svg class="h-4 w-4 animate-spin" fill="none" viewBox="0 0 24 24">
									<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
									<path
										class="opacity-75"
										fill="currentColor"
										d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
									></path>
								</svg>
							{:else}
								<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M12 6v6m0 0v6m0-6h6m-6 0H6"
									/>
								</svg>
							{/if}
							{replies.loading ? '로딩 중...' : '답글 더 보기'}
						</button>
					</div>
				{/if}
			{/if}
		</div>
	</div>
</div>

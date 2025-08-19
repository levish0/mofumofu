<script lang="ts">
	import { createComment } from '$lib/api/comment/commentApi';
	import type { CommentInfo, CreateCommentRequest } from '$lib/api/comment/types';
	import { authStore } from '$lib/stores/auth.svelte';
	import { userStore } from '$lib/stores/user.svelte';

	interface Props {
		postId: string;
		parentId?: string;
		placeholder?: string;
		onSubmit?: (comment: CommentInfo) => void;
		onCancel?: () => void;
		compact?: boolean;
		mentionText?: string;
	}

	let { 
		postId, 
		parentId, 
		placeholder = "댓글을 작성하세요...",
		onSubmit,
		onCancel,
		compact = false,
		mentionText = ""
	}: Props = $props();

	let content = $state('');
	let isSubmitting = $state(false);
	let textareaElement = $state<HTMLTextAreaElement>();

	const maxLength = 500;
	const remainingChars = $derived(maxLength - content.length);

	// 자동 높이 조절
	const adjustHeight = () => {
		if (textareaElement) {
			textareaElement.style.height = 'auto';
			textareaElement.style.height = textareaElement.scrollHeight + 'px';
		}
	};

	const handleInput = (e: Event) => {
		const target = e.target as HTMLTextAreaElement;
		content = target.value;
		adjustHeight();
	};

	const handleSubmit = async () => {
		if (!content.trim() || isSubmitting || !userStore.user) return;

		try {
			isSubmitting = true;

			const request: CreateCommentRequest = {
				content: content.trim(),
				post_id: postId,
				parent_id: parentId || null
			};

			await createComment(request);

			// 성공 시 폼 초기화
			content = '';
			adjustHeight();

			// 부모 컴포넌트에 성공 알림 (실제 댓글 정보는 새로고침으로 받아올 예정)
			if (onSubmit) {
				// 임시 댓글 객체 생성 (실제로는 서버에서 받아와야 함)
				const tempComment: CommentInfo = {
					id: crypto.randomUUID(),
					post_id: postId,
					content: content.trim(),
					parent_id: parentId || null,
					user_id: null, // UserInfoResponse에 id가 없으므로 null로 설정
					user_handle: userStore.user?.handle || null,
					user_name: userStore.user?.name || null,
					user_profile_image: userStore.user?.profile_image || null,
					like_count: 0,
					reply_count: 0,
					is_liked: false,
					is_deleted: false,
					created_at: new Date().toISOString(),
					updated_at: null
				};
				onSubmit(tempComment);
			}
		} catch (error) {
			console.error('Failed to submit comment:', error);
		} finally {
			isSubmitting = false;
		}
	};

	const handleKeyDown = (e: KeyboardEvent) => {
		if (e.key === 'Enter' && (e.ctrlKey || e.metaKey)) {
			e.preventDefault();
			handleSubmit();
		}
	};

	const handleCancel = () => {
		content = '';
		adjustHeight();
		if (onCancel) {
			onCancel();
		}
	};
</script>

<div class="comment-form">
	{#if !userStore.user}
		<div class="flex items-center justify-center py-8 text-mofu-light-400 dark:text-mofu-dark-400 text-sm">
			로그인이 필요합니다.
		</div>
	{:else}
		<div class="flex gap-3" class:gap-2={compact}>
			<!-- 사용자 프로필 이미지 -->
			<div class="flex-shrink-0">
				{#if userStore.user.profile_image}
					<img 
						src={userStore.user.profile_image} 
						alt={userStore.user.name}
						class="rounded-full object-cover"
						class:w-8={compact}
						class:h-8={compact}
						class:w-10={!compact}
						class:h-10={!compact}
					/>
				{:else}
					<div 
						class="rounded-full bg-mofu-light-600 dark:bg-mofu-dark-600 flex items-center justify-center"
						class:w-8={compact}
						class:h-8={compact}
						class:w-10={!compact}
						class:h-10={!compact}
					>
						<span class="text-mofu-light-200 dark:text-mofu-dark-200 font-medium" class:text-xs={compact} class:text-sm={!compact}>
							{userStore.user.name.charAt(0).toUpperCase()}
						</span>
					</div>
				{/if}
			</div>

			<!-- 댓글 입력 영역 -->
			<div class="flex-1 min-w-0">
				<div class="bg-mofu-light-800 dark:bg-mofu-dark-800 rounded-lg border border-gray-200 dark:border-gray-700 focus-within:border-blue-500 focus-within:ring-1 focus-within:ring-blue-500 transition-colors">
					<!-- 텍스트 영역 -->
					<textarea
						bind:this={textareaElement}
						value={content}
						oninput={handleInput}
						onkeydown={handleKeyDown}
						{placeholder}
						disabled={isSubmitting}
						maxlength={maxLength}
						class="w-full p-3 bg-transparent border-0 resize-none focus:ring-0 focus:outline-none text-sm text-black dark:text-white placeholder-mofu-light-400 dark:placeholder-mofu-dark-400"
						class:p-2={compact}
						class:text-xs={compact}
						rows="1"
					></textarea>

					<!-- 하단 바 -->
					<div class="flex items-center justify-between px-3 pb-3" class:px-2={compact} class:pb-2={compact}>
						<div class="flex items-center gap-2 text-xs text-mofu-light-400 dark:text-mofu-dark-400">
							<!-- 문자 수 카운터 -->
							<span class="text-xs" class:text-red-500={remainingChars < 50}>
								{remainingChars}/{maxLength}
							</span>
							
							<!-- 단축키 힌트 -->
							{#if !compact}
								<span class="text-xs text-mofu-light-500 dark:text-mofu-dark-500">
									Ctrl+Enter로 등록
								</span>
							{/if}
						</div>

						<!-- 액션 버튼들 -->
						<div class="flex items-center gap-2">
							{#if onCancel}
								<button
									type="button"
									onclick={handleCancel}
									disabled={isSubmitting}
									class="px-3 py-1 text-xs text-gray-600 hover:text-gray-800 disabled:text-gray-400 transition-colors"
									class:px-2={compact}
								>
									취소
								</button>
							{/if}
							
							<button
								type="button"
								onclick={handleSubmit}
								disabled={!content.trim() || isSubmitting || remainingChars < 0}
								class="px-4 py-1.5 bg-blue-500 text-white text-xs rounded-md hover:bg-blue-600 disabled:bg-gray-300 disabled:cursor-not-allowed transition-colors"
								class:px-3={compact}
								class:py-1={compact}
							>
								{isSubmitting ? '등록 중...' : '등록'}
							</button>
						</div>
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>

<style>
	textarea {
		min-height: 44px;
		max-height: 200px;
	}
</style>
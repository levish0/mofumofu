import { getReplies } from '$lib/api/comment/commentApi';
import type { CommentInfo, GetRepliesRequest } from '$lib/api/comment/types';

interface UseRepliesState {
	replies: CommentInfo[];
	currentPage: number;
	loading: boolean;
	hasMore: boolean;
	totalCount: number;
	expanded: boolean;
}

export function useReplies(parentCommentId: string, pageSize: number = 3) {
	let state = $state<UseRepliesState>({
		replies: [],
		currentPage: 1,
		loading: false,
		hasMore: false,
		totalCount: 0,
		expanded: false
	});

	const loadReplies = async (page: number = 1, reset: boolean = false) => {
		if (state.loading) return;

		try {
			state.loading = true;

			const request: GetRepliesRequest = {
				parent_comment_id: parentCommentId,
				page,
				per_page: pageSize
			};

			const response = await getReplies(request);

			if (reset) {
				state.replies = response.replies;
			} else {
				state.replies.push(...response.replies);
			}

			state.currentPage = response.page;
			state.hasMore = response.has_next;
			state.totalCount = response.total_count;
		} catch (error) {
			console.error('Failed to load replies:', error);
		} finally {
			state.loading = false;
		}
	};

	const loadMoreReplies = async () => {
		if (state.hasMore && !state.loading) {
			await loadReplies(state.currentPage + 1, false);
		}
	};

	const toggleReplies = async () => {
		if (!state.expanded && state.replies.length === 0) {
			await loadReplies(1, true);
		}
		state.expanded = !state.expanded;
	};

	const addReply = (newReply: CommentInfo) => {
		state.replies.unshift(newReply);
		state.totalCount++;
		if (!state.expanded) {
			state.expanded = true;
		}
	};

	const refreshReplies = async () => {
		if (state.expanded) {
			await loadReplies(1, true);
		}
	};

	return {
		get replies() { return state.replies; },
		get loading() { return state.loading; },
		get hasMore() { return state.hasMore; },
		get totalCount() { return state.totalCount; },
		get expanded() { return state.expanded; },
		loadReplies,
		loadMoreReplies,
		toggleReplies,
		addReply,
		refreshReplies
	};
}
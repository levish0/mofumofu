import { searchPosts } from '$lib/api/post/postApi';
import type { PostSortOrder } from '$lib/api/post/types';
import { profilePostsStore } from '$lib/stores/profilePosts.svelte';

interface UseProfilePostsDataConfig {
	pageSize?: number;
	pagesAround?: number;
}

export function useProfilePostsData(config: UseProfilePostsDataConfig = {}) {
	const PAGE_SIZE = config.pageSize ?? 12;
	const PAGES_AROUND = config.pagesAround ?? 2;

	// 초기 데이터 로드
	const loadInitialPosts = async (userHandle: string, sortOrder: PostSortOrder = 'latest') => {
		if (!userHandle) return;
		
		try {
			profilePostsStore.setInitialLoading(true);
			profilePostsStore.setUserHandle(userHandle);
			profilePostsStore.setSortOrder(sortOrder);

			const response = await searchPosts({
				user_handle: userHandle,
				sort: sortOrder,
				page_size: PAGE_SIZE,
				target_page: 1
			});

			profilePostsStore.setPosts(response.posts);
			profilePostsStore.setHasMore(response.has_more);
			profilePostsStore.setCurrentPage(2); // Next page after first load
			profilePostsStore.setInitialized(true);
		} catch (error) {
			console.error('Failed to load profile posts:', error);
			profilePostsStore.setPosts([]);
			profilePostsStore.setHasMore(false);
		} finally {
			profilePostsStore.setInitialLoading(false);
		}
	};

	// 더 많은 포스트 로드
	const loadMorePosts = async () => {
		if (profilePostsStore.loading || !profilePostsStore.hasMore || !profilePostsStore.userHandle) {
			return;
		}

		profilePostsStore.setLoading(true);

		try {
			const response = await searchPosts({
				user_handle: profilePostsStore.userHandle,
				sort: profilePostsStore.sortOrder,
				page_size: PAGE_SIZE,
				target_page: profilePostsStore.currentPage,
				pages_around: PAGES_AROUND
			});

			// 중복 제거 후 추가
			const existingIds = new Set(profilePostsStore.posts.map((post) => post.id));
			const newPosts = response.posts.filter((post) => !existingIds.has(post.id));

			profilePostsStore.addPosts(newPosts);
			profilePostsStore.setCurrentPage(profilePostsStore.currentPage + 1);
			profilePostsStore.setHasMore(response.has_more);
		} catch (error) {
			console.error('Failed to load more profile posts:', error);
		} finally {
			profilePostsStore.setLoading(false);
		}
	};

	// 정렬 변경 시 데이터 리로드
	const changeSortOrder = async (sortOrder: PostSortOrder) => {
		if (profilePostsStore.sortOrder === sortOrder || !profilePostsStore.userHandle) {
			return;
		}

		profilePostsStore.reset(profilePostsStore.userHandle, sortOrder);
		await loadInitialPosts(profilePostsStore.userHandle, sortOrder);
	};

	return {
		// Store getters
		get posts() {
			return profilePostsStore.posts;
		},
		get loading() {
			return profilePostsStore.loading;
		},
		get initialLoading() {
			return profilePostsStore.initialLoading;
		},
		get hasMore() {
			return profilePostsStore.hasMore;
		},
		get currentPage() {
			return profilePostsStore.currentPage;
		},
		get sortOrder() {
			return profilePostsStore.sortOrder;
		},
		get userHandle() {
			return profilePostsStore.userHandle;
		},
		get initialized() {
			return profilePostsStore.initialized;
		},

		// Actions
		loadInitialPosts,
		loadMorePosts,
		changeSortOrder
	};
}
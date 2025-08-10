// src/lib/hooks/usePostsData.svelte.ts

import { getPosts, getPostsAroundPage, searchPosts } from '$lib/api/post/postApi';
import type {
	PostSortOrder,
	GetPostsRequest,
	GetPostsAroundPageRequest,
	SearchPostsRequest
} from '$lib/api/post/types';
import { postsStore } from '$lib/stores/posts.svelte';
import { onMount } from 'svelte';

interface UsePostsDataConfig {
	pageSize?: number;
	pagesAround?: number;
}

export function usePostsData(config: UsePostsDataConfig = {}) {
	const PAGE_SIZE = config.pageSize ?? 8;
	const PAGES_AROUND = config.pagesAround ?? 2;

	// Store 상태를 reactive하게 사용
	const posts = $derived(postsStore.posts);
	const loading = $derived(postsStore.loading);
	const initialLoading = $derived(postsStore.initialLoading);
	const hasMore = $derived(postsStore.hasMore);
	const currentPage = $derived(postsStore.currentPage);
	const filter = $derived(postsStore.filter);
	const isSearchMode = $derived(postsStore.isSearchMode);

	// API 파라미터 매핑 함수들
	const mapSortByToApiSort = (sortBy: string): PostSortOrder => {
		switch (sortBy) {
			case 'recent':
				return 'latest';
			case 'oldest':
				return 'oldest';
			case 'popular':
			case 'comments':
			case 'views':
				return 'popular';
			default:
				return 'latest';
		}
	};

	const mapTimeRangeToDate = (timeRange: string) => {
		const now = new Date();
		const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());

		switch (timeRange) {
			case 'today':
				return { date_from: today.toISOString() };
			case 'week':
				const weekAgo = new Date(today.getTime() - 7 * 24 * 60 * 60 * 1000);
				return { date_from: weekAgo.toISOString() };
			case 'month':
				const monthAgo = new Date(today.getFullYear(), today.getMonth() - 1, today.getDate());
				return { date_from: monthAgo.toISOString() };
			case 'year':
				const yearAgo = new Date(today.getFullYear() - 1, today.getMonth(), today.getDate());
				return { date_from: yearAgo.toISOString() };
			default:
				return {};
		}
	};

	// 파생된 값들
	const apiSort = $derived(mapSortByToApiSort(filter.sortBy));
	const dateRange = $derived(mapTimeRangeToDate(filter.timeRange));

	// API 호출을 통합하는 헬퍼 함수
	const buildApiParams = (targetPage: number, isLoadMore = false) => {
		const currentFilter = postsStore.filter;
		const currentApiSort = mapSortByToApiSort(currentFilter.sortBy);
		const currentDateRange = mapTimeRangeToDate(currentFilter.timeRange);

		const baseParams = {
			page_size: PAGE_SIZE,
			sort: currentApiSort,
			...currentDateRange
		};

		if (postsStore.isSearchMode) {
			return {
				query: currentFilter.keyword || null,
				hashtags: currentFilter.tags.length > 0 ? currentFilter.tags : null,
				target_page: targetPage,
				pages_around: targetPage > 1 || isLoadMore ? PAGES_AROUND : null,
				...baseParams
			};
		} else {
			if (targetPage > 1 || isLoadMore) {
				return {
					target_page: targetPage,
					pages_around: PAGES_AROUND,
					...baseParams
				};
			} else {
				return {
					page: 1,
					...baseParams
				};
			}
		}
	};

	// API 호출 함수 선택
	const callApi = async (targetPage: number, isLoadMore = false) => {
		const params = buildApiParams(targetPage, isLoadMore);

		if (postsStore.isSearchMode) {
			return await searchPosts(params as SearchPostsRequest);
		} else if (targetPage > 1 || isLoadMore) {
			return await getPostsAroundPage(params as GetPostsAroundPageRequest);
		} else {
			return await getPosts(params as GetPostsRequest);
		}
	};

	// 초기 데이터 로드
	const loadInitialPosts = async () => {
		try {
			postsStore.setInitialLoading(true);

			const targetPage = postsStore.targetPage > 1 ? postsStore.targetPage : 1;
			const response = await callApi(targetPage);

			postsStore.setPosts(response.posts);
			postsStore.setHasMore(response.has_more);
			postsStore.setCurrentPage(response.current_page);
		} catch (error) {
			console.error('Failed to load initial posts:', error);
			postsStore.setPosts([]);
			postsStore.setHasMore(false);
		} finally {
			postsStore.setInitialLoading(false);
		}
	};

	// 더 많은 포스트 로드
	const loadMorePosts = async () => {
		if (postsStore.loading || !postsStore.hasMore) return;

		postsStore.setLoading(true);

		try {
			const nextPage = postsStore.currentPage + 1;
			const response = await callApi(nextPage, true);

			// 중복 제거 후 추가
			const existingIds = new Set(postsStore.posts.map((post) => post.id));
			const newPosts = response.posts.filter((post) => !existingIds.has(post.id));

			postsStore.addPosts(newPosts);
			postsStore.setCurrentPage(nextPage);
			postsStore.setTargetPage(nextPage);
			postsStore.setHasMore(response.has_more);
		} catch (error) {
			console.error('Failed to load more posts:', error);
		} finally {
			postsStore.setLoading(false);
		}
	};

	// 필터 변경 시 데이터 리로드
	const reloadWithNewFilter = () => {
		postsStore.setTargetPage(1);
		loadInitialPosts();
	};

	// 컴포넌트 마운트 시 초기 로드
	onMount(() => {
		loadInitialPosts();
	});

	// Reactive한 객체 직접 반환
	const reactiveState = {
		get posts() {
			return postsStore.posts;
		},
		get loading() {
			return postsStore.loading;
		},
		get initialLoading() {
			return postsStore.initialLoading;
		},
		get hasMore() {
			return postsStore.hasMore;
		},
		get currentPage() {
			return postsStore.currentPage;
		},
		get filter() {
			return postsStore.filter;
		},
		get isSearchMode() {
			return postsStore.isSearchMode;
		}
	};

	return {
		...reactiveState,
		// 액션
		loadInitialPosts,
		loadMorePosts,
		reloadWithNewFilter
	};
}

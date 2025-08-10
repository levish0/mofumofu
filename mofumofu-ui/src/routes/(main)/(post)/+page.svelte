<script lang="ts">
	import PostList from '$lib/components/post/PostList.svelte';
	import { getPosts, getPostsAroundPage, searchPosts } from '$lib/api/post/postApi';
	import type { PostListItem, PostSortOrder } from '$lib/api/post/types';
	import { postsStore } from '$lib/stores/posts.svelte';
	import { onMount } from 'svelte';

	const PAGE_SIZE = 8;
	const skeletonCount = PAGE_SIZE - 4;
	const PAGES_AROUND = 2; // target_page 주변으로 가져올 페이지 수

	// SearchPanel의 sortBy를 API의 PostSortOrder로 매핑
	const mapSortByToApiSort = (sortBy: string): PostSortOrder => {
		switch (sortBy) {
			case 'recent':
				return 'latest';
			case 'oldest':
				return 'oldest';
			case 'popular':
			case 'comments':
			case 'views':
				return 'popular'; // 임시로 popular로 매핑
			default:
				return 'latest';
		}
	};

	// SearchPanel의 timeRange를 날짜 범위로 매핑
	const mapTimeRangeToDate = (timeRange: string): { date_from?: string; date_to?: string } => {
		const now = new Date();
		const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());

		switch (timeRange) {
			case 'today':
				return { date_from: today.toISOString() };
			case 'week':
				const weekAgo = new Date(today);
				weekAgo.setDate(weekAgo.getDate() - 7);
				return { date_from: weekAgo.toISOString() };
			case 'month':
				const monthAgo = new Date(today);
				monthAgo.setMonth(monthAgo.getMonth() - 1);
				return { date_from: monthAgo.toISOString() };
			case 'year':
				const yearAgo = new Date(today);
				yearAgo.setFullYear(yearAgo.getFullYear() - 1);
				return { date_from: yearAgo.toISOString() };
			default:
				return {};
		}
	};

	// store의 상태를 reactive하게 사용
	let posts = $derived(postsStore.posts);
	let loading = $derived(postsStore.loading);
	let initialLoading = $derived(postsStore.initialLoading);
	let hasMore = $derived(postsStore.hasMore);
	let currentPage = $derived(postsStore.currentPage);
	let filter = $derived(postsStore.filter);
	let isSearchMode = $derived(postsStore.isSearchMode);

	// 파생된 API 정렬 옵션
	let apiSort = $derived(mapSortByToApiSort(filter.sortBy));
	let dateRange = $derived(mapTimeRangeToDate(filter.timeRange));

	// 초기 데이터 로드 또는 around page로 로드
	const loadInitialPosts = async () => {
		try {
			postsStore.setInitialLoading(true);

			let response;

			if (isSearchMode) {
				// 검색 모드일 때는 search API 사용
				response = await searchPosts({
					query: filter.keyword || null,
					hashtags: filter.tags.length > 0 ? filter.tags : null,
					sort: apiSort,
					page_size: PAGE_SIZE,
					target_page: postsStore.targetPage > 1 ? postsStore.targetPage : 1,
					pages_around: postsStore.targetPage > 1 ? PAGES_AROUND : null,
					...dateRange
				});
			} else {
				// 일반 모드
				if (postsStore.targetPage > 1) {
					response = await getPostsAroundPage({
						target_page: postsStore.targetPage,
						page_size: PAGE_SIZE,
						pages_around: PAGES_AROUND,
						sort: apiSort
					});
				} else {
					response = await getPosts({
						page: 1,
						page_size: PAGE_SIZE,
						sort: apiSort
					});
				}
			}

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

	// 더 많은 포스트 로드 - search API 또는 around API 활용
	const onLoadMore = async () => {
		if (postsStore.loading || !postsStore.hasMore) return;

		postsStore.setLoading(true);

		try {
			const nextPage = postsStore.currentPage + 1;
			let response;

			if (isSearchMode) {
				// 검색 모드일 때는 search API 사용
				response = await searchPosts({
					query: filter.keyword || null,
					hashtags: filter.tags.length > 0 ? filter.tags : null,
					sort: apiSort,
					page_size: PAGE_SIZE,
					target_page: nextPage,
					pages_around: PAGES_AROUND,
					...dateRange
				});
			} else {
				// 일반 모드일 때는 around API 사용
				response = await getPostsAroundPage({
					target_page: nextPage,
					page_size: PAGE_SIZE,
					pages_around: PAGES_AROUND,
					sort: apiSort
				});
			}

			// 새로운 포스트들만 필터링해서 추가
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

	// 컴포넌트 마운트 시 초기 데이터 로드
	onMount(() => {
		loadInitialPosts();
	});

	// 이전 정렬 값 추적
	let prevSortBy = $state('');

	// 필터 변경 시 posts 리로드
	$effect(() => {
		// 필터가 실제로 변경되었는지 확인 (검색어, 태그, 정렬, 기간)
		const currentSortBy = filter.sortBy;
		if (currentSortBy !== prevSortBy) {
			prevSortBy = currentSortBy;

			// 초기 로딩이 완료된 후에만 리로드
			if (!postsStore.initialLoading) {
				postsStore.setTargetPage(1); // 필터 변경 시 첫 페이지부터 시작
				loadInitialPosts();
			}
		}
	});

	// 이전 검색 상태 추적
	let prevKeyword = $state('');
	let prevTags = $state<string[]>([]);
	let searchTimeout: number | null = null;

	// 검색어나 태그 변경 시 debounced 검색
	$effect(() => {
		const currentKeyword = filter.keyword;
		const currentTags = filter.tags;
		const keywordChanged = currentKeyword !== prevKeyword;
		const tagsChanged = JSON.stringify(currentTags) !== JSON.stringify(prevTags);

		if (keywordChanged || tagsChanged) {
			prevKeyword = currentKeyword;
			prevTags = [...currentTags];

			// 기존 timeout 취소
			if (searchTimeout) {
				clearTimeout(searchTimeout);
			}

			// 검색어가 있으면 500ms 후 검색, 없으면 즉시 일반 모드로
			if (currentKeyword || currentTags.length > 0) {
				searchTimeout = window.setTimeout(() => {
					if (!postsStore.initialLoading) {
						postsStore.setTargetPage(1);
						loadInitialPosts();
					}
				}, 500);
			} else {
				// 검색어와 태그가 모두 비어있으면 즉시 일반 모드로
				if (!postsStore.initialLoading) {
					postsStore.setTargetPage(1);
					loadInitialPosts();
				}
			}
		}
	});
</script>

<PostList {posts} {loading} {onLoadMore} {hasMore} {skeletonCount} />

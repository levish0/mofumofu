<script lang="ts">
	import PostList from '$lib/components/post/PostList.svelte';
	import { usePostsData } from '$lib/hooks/posts/usePostsData.svelte';
	import { usePostsFilter } from '$lib/hooks/posts/usePostsFilter.svelte';
	import { postsStore } from '$lib/stores/posts.svelte';

	const PAGE_SIZE = 12;
	const skeletonCount = 4;

	// 커스텀 훅 사용 (액션만 사용)
	const { loadMorePosts, reloadWithNewFilter } = usePostsData({
		pageSize: PAGE_SIZE
	});

	// Store에서 직접 reactive 값 가져오기
	const posts = $derived(postsStore.posts);
	const loading = $derived(postsStore.loading);
	const hasMore = $derived(postsStore.hasMore);

	// 필터 변경 감지
	usePostsFilter({
		onFilterChange: reloadWithNewFilter,
		debounceMs: 400
	});
</script>

<PostList {posts} {loading} onLoadMore={loadMorePosts} {hasMore} {skeletonCount} />

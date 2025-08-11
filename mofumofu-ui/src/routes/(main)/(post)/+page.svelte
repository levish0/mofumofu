<script lang="ts">
	import PostList from '$lib/components/post/PostList.svelte';
	import { usePostsData } from '$lib/hooks/posts/usePostsData.svelte';
	import { postsStore } from '$lib/stores/posts.svelte';

	const PAGE_SIZE = 15;
	const skeletonCount = 5;

	// 커스텀 훅 사용
	const { loadMorePosts } = usePostsData({
		pageSize: PAGE_SIZE,
		sortOrder: 'popular'
	});

	// Store에서 직접 reactive 값 가져오기
	const posts = $derived(postsStore.posts);
	const loading = $derived(postsStore.loading);
	const hasMore = $derived(postsStore.hasMore);
</script>

<PostList {posts} {loading} onLoadMore={loadMorePosts} {hasMore} {skeletonCount} />

<script lang="ts">
	import PostCard from './PostCard.svelte';
	import PostCardSkeleton from './PostCardSkeleton.svelte';
	import { useInfiniteScroll } from '$lib/hooks/useInfiniteScroll.svelte';

	interface Author {
		name: string;
		avatar: string;
	}

	interface Card {
		id: number;
		image?: string;
		title: string;
		summary: string;
		date: string;
		comments: number;
		views: string;
		author: Author;
		likes: number;
	}

	let {
		cards = [],
		loading = false,
		onLoadMore,
		hasMore = true,
		skeletonCount = 8
	}: {
		cards: Card[];
		loading: boolean;
		onLoadMore: () => Promise<void>;
		hasMore?: boolean;
		skeletonCount?: number;
	} = $props();

	// 무한 스크롤 훅 사용
	useInfiniteScroll({
		onLoadMore,
		isLoading: () => loading,
		hasMore: () => hasMore,
		threshold: 50
	});
</script>

<div class="min-h-screen">
	<div class="grid grid-cols-1 gap-x-5 gap-y-4 pb-20 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
		{#each cards as card (card.id)}
			<PostCard
				image={card.image}
				title={card.title}
				summary={card.summary}
				date={card.date}
				comments={card.comments}
				views={card.views}
				author_name={card.author.name}
				author_avatar={card.author.avatar}
				likes={card.likes}
			/>
		{/each}

		<!-- Skeletons -->
		{#if loading}
			{#each Array(skeletonCount) as _, i}
				<PostCardSkeleton />
			{/each}
		{/if}
	</div>

	<!-- 끝 -->
	{#if !hasMore && !loading}
		<div class="dark:text-mofu-dark-300 pb-20 text-center text-lg font-bold">모든 포스트를 확인하셨습니다 ✨</div>
	{:else if cards.length > 20 && !loading && hasMore}
		<div class="dark:text-mofu-dark-300 pb-20 text-center text-lg font-bold">
			스크롤하여 더 많은 포스트를 확인하세요 📜
		</div>
	{/if}
</div>

<!-- src/lib/components/search/SearchPanel.svelte -->
<script lang="ts">
	import { postsStore } from '$lib/stores/posts.svelte';
	import { getTrendingHashtags } from '$lib/api/hashtag/hashtagApi';
	import { Badge } from '../ui/badge';
	import { onMount } from 'svelte';

	let { isVisible, isAtTop } = $props();

	let trendingHashtags = $state<string[]>([]);
	let isLoadingTags = $state(true);

	// store의 filter 상태를 reactive하게 사용
	let keyword = $derived(postsStore.filter.keyword);
	let selected = $derived(postsStore.filter.tags);
	let sortBy = $derived(postsStore.filter.sortBy);
	let timeRange = $derived(postsStore.filter.timeRange);

	// 필터 변경 함수들
	const updateKeyword = (value: string) => {
		postsStore.setFilter({ keyword: value });
	};

	const toggleTag = (tag: string) => {
		const newTags = selected.includes(tag) ? selected.filter((t) => t !== tag) : [...selected, tag];
		postsStore.setFilter({ tags: newTags });
	};

	const updateSortBy = (value: string) => {
		postsStore.setFilter({ sortBy: value });
	};

	const updateTimeRange = (value: string) => {
		postsStore.setFilter({ timeRange: value });
	};

	// trending hashtags 로드
	async function loadTrendingHashtags() {
		try {
			isLoadingTags = true;
			const response = await getTrendingHashtags({ days: 7, limit: 12 });
			trendingHashtags = response.hashtags;
		} catch (error) {
			console.error('Failed to load trending hashtags:', error);
			// 에러 시 더미 태그 사용
			trendingHashtags = ['React', 'TypeScript', 'Next.js', 'Svelte', 'Zustand', 'UX'];
		} finally {
			isLoadingTags = false;
		}
	}

	// Calculate the top position based on navbar state
	const topPosition = $derived(isVisible() ? '68px' : '8px');

	onMount(() => {
		loadTrendingHashtags();
	});
</script>

<div class="sticky w-full space-y-6 transition-all duration-100 ease-out" style="top: {topPosition}">
	<h2 class="text-mofu-dark-100 text-xl font-bold">검색 & 필터</h2>

	<!-- 검색창 -->
	<div>
		<label for="search" class="sr-only">검색어</label>
		<input
			id="search"
			type="text"
			value={keyword}
			oninput={(e) => updateKeyword(e.currentTarget.value)}
			placeholder="제목 / 요약 검색"
			class="border-mofu-dark-600 bg-mofu-dark-700 placeholder-mofu-dark-300 w-full
		              rounded-lg px-3 py-2 text-sm
		              text-white outline-none"
		/>
	</div>

	<!-- 태그 필터 -->
	<div>
		<h3 class="text-mofu-dark-100 mb-2 text-sm font-semibold">인기 태그</h3>
		{#if isLoadingTags}
			<!-- 로딩 스켈레톤 -->
			<div class="flex flex-wrap gap-2">
				{#each Array(6) as _}
					<div class="shimmer h-6 w-16 rounded-full bg-mofu-dark-800"></div>
				{/each}
			</div>
		{:else}
			<div class="flex flex-wrap gap-2">
				{#each trendingHashtags as tag}
					<Badge
						variant="secondary"
						class="cursor-pointer text-xs {selected.includes(tag) 
							? 'bg-mofu text-mofu-dark-950 hover:bg-mofu/90' 
							: 'bg-mofu-dark-800 text-mofu hover:bg-mofu-dark-700 hover:text-mofu'} transition-colors"
						onclick={() => toggleTag(tag)}
					>
						#{tag}
					</Badge>
				{/each}
			</div>
		{/if}
	</div>

	<!-- 정렬 -->
	<div class="space-y-2">
		<h3 class="text-mofu-dark-100 text-sm font-semibold">정렬 기준</h3>
		<select
			value={sortBy}
			onchange={(e) => updateSortBy(e.currentTarget.value)}
			class="border-mofu-dark-600 bg-mofu-dark-700 text-mofu-dark-300 w-full rounded-lg px-3 py-2 text-sm"
		>
			<option value="recent">최신순</option>
			<option value="oldest">오래된순</option>
			<option value="popular">좋아요순</option>
			<option value="comments">댓글 많은 순</option>
			<option value="views">조회수 순</option>
		</select>

		<h3 class="text-mofu-dark-100 text-sm font-semibold">기간</h3>
		<select
			value={timeRange}
			onchange={(e) => updateTimeRange(e.currentTarget.value)}
			class="border-mofu-dark-600 bg-mofu-dark-700 text-mofu-dark-300 w-full rounded-lg px-3 py-2 text-sm"
		>
			<option value="all">전체</option>
			<option value="today">오늘</option>
			<option value="week">이번 주</option>
			<option value="month">이번 달</option>
			<option value="year">올해</option>
		</select>
	</div>
</div>

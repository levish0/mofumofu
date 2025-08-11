<script lang="ts">
	import PostCard from '../post/PostCard.svelte';
	import PostCardSkeleton from '../post/PostCardSkeleton.svelte';
	import type { PostSortOrder } from '$lib/api/post/types';
	import { Button } from '../ui/button';
	import { Badge } from '../ui/badge';
	import * as Select from '../ui/select';
	import { useInfiniteScroll } from '$lib/hooks/ui/useInfiniteScroll.svelte';
	import { useProfilePostsData } from '$lib/hooks/profile/useProfilePostsData.svelte';
	import { profilePostsStore } from '$lib/stores/profilePosts.svelte';
	import { getContext } from 'svelte';

	type Props = {
		profile: {
			handle: string;
			name: string;
			profile_image?: string;
		};
	};

	const { profile }: Props = $props();

	const PAGE_SIZE = 12;
	const skeletonCount = 4;

	const sortOptions: { value: PostSortOrder; label: string }[] = [
		{ value: 'latest', label: '최신순' },
		{ value: 'oldest', label: '오래된순' },
		{ value: 'popular', label: '인기순' }
	];

	// 현재 로드된 포스트들의 해시태그
	let availableHashtags = $state<string[]>([]);
	let selectedTags = $state<string[]>([]);

	// 전체 포스트들에서 해시태그 추출
	function extractHashtagsFromPosts() {
		const allHashtags = new Set<string>();
		allPosts.forEach((post) => {
			post.hashtags.forEach((tag) => {
				if (tag.trim()) {
					allHashtags.add(tag.trim());
				}
			});
		});
		availableHashtags = Array.from(allHashtags); // 모든 태그 표시
	}

	// 태그 토글 함수 - 클라이언트 사이드 필터링
	function toggleTag(tag: string) {
		if (selectedTags.includes(tag)) {
			selectedTags = selectedTags.filter((t) => t !== tag);
		} else {
			selectedTags = [...selectedTags, tag];
		}
		// 클라이언트 사이드 필터링이므로 API 호출 없음
	}

	// 프로필 포스트 데이터 훅 사용
	const { loadInitialPosts, loadMorePosts, changeSortOrder } = useProfilePostsData({
		pageSize: PAGE_SIZE,
		pagesAround: 2
	});

	// Store에서 직접 reactive 값 가져오기
	const allPosts = $derived(profilePostsStore.posts);
	const loading = $derived(profilePostsStore.loading);
	const initialLoading = $derived(profilePostsStore.initialLoading);
	const hasMore = $derived(profilePostsStore.hasMore);
	const currentSort = $derived(profilePostsStore.sortOrder);
	const storeUserHandle = $derived(profilePostsStore.userHandle);
	const initialized = $derived(profilePostsStore.initialized);

	// 필터링된 포스트들
	const posts = $derived((() => {
		if (selectedTags.length === 0) {
			return allPosts; // 선택된 태그가 없으면 모든 포스트 표시
		}
		
		// 선택된 태그 중 하나라도 포함하는 포스트만 필터링
		return allPosts.filter(post => {
			return selectedTags.some(selectedTag => 
				post.hashtags.some(postTag => 
					postTag.toLowerCase().includes(selectedTag.toLowerCase())
				)
			);
		});
	})());

	// 정렬 변경 핸들러
	function handleSortChange(sort: PostSortOrder) {
		changeSortOrder(sort);
	}

	// 무한 스크롤 훅 사용
	useInfiniteScroll({
		onLoadMore: loadMorePosts,
		isLoading: () => loading,
		hasMore: () => hasMore,
		threshold: 100
	});

	// 전체 포스트가 변경될 때마다 해시태그 추출
	$effect(() => {
		if (allPosts.length > 0) {
			extractHashtagsFromPosts();
		}
	});

	// navbar context 가져오기
	type NavbarContext = {
		isVisible: () => boolean;
		isAtTop: () => boolean;
	};

	const navbar = getContext<NavbarContext>('navbar');

	// navbar 상태에 따른 sticky top 위치 계산
	const stickyTopPosition = $derived(navbar?.isVisible() ? '60px' : '0px');

	// 프로필이 변경되거나 초기 로드
	$effect(() => {
		if (profile.handle && (!initialized || storeUserHandle !== profile.handle)) {
			loadInitialPosts(profile.handle, 'latest');
		}
	});
</script>

<div class="space-y-6">
	<!-- Sticky Hashtags & Sort Section -->
	<div class="bg-mofu-dark-900 sticky z-20 py-4 transition-all duration-100 ease-out" style="top: {stickyTopPosition}">
		<div class="flex flex-col gap-4 lg:flex-row lg:items-center lg:justify-between">
			<!-- Left: Hashtags -->
			<div class="flex-1">
				{#if availableHashtags.length > 0}
					<div class="flex flex-wrap gap-2">
						{#each availableHashtags as tag}
							<Badge
								variant="secondary"
								class="cursor-pointer text-xs {selectedTags.includes(tag)
									? 'bg-mofu text-mofu-dark-950 hover:bg-mofu/90'
									: 'bg-mofu-dark-800 text-mofu hover:bg-mofu-dark-700 hover:text-mofu'} transition-colors"
								onclick={() => toggleTag(tag)}
							>
								#{tag}
							</Badge>
						{/each}
					</div>
				{:else}
					<div class="text-mofu-dark-400 text-sm">필터할 태그가 없습니다.</div>
				{/if}
			</div>

			<!-- Right: Sort Dropdown -->
			<div class="lg:w-48">
				<Select.Root type="single">
					<Select.Trigger
						class="border-mofu-dark-600 bg-mofu-dark-700 text-mofu-dark-200 focus-visible:border-mofu focus-visible:ring-mofu w-full"
					>
						{sortOptions.find((o) => o.value === currentSort)?.label || '최신순'}
					</Select.Trigger>
					<Select.Content class="bg-mofu-dark-700 border-mofu-dark-600">
						{#each sortOptions as option}
							<Select.Item
								value={option.value}
								class="text-mofu-dark-200 focus:bg-mofu-dark-600"
								onclick={() => handleSortChange(option.value)}
							>
								{option.label}
							</Select.Item>
						{/each}
					</Select.Content>
				</Select.Root>
			</div>
		</div>
	</div>

	<!-- Posts Grid -->
	<div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
		{#each posts as post (`${post.user_handle}-${post.slug}`)}
			<PostCard {post} />
		{/each}

		<!-- Loading skeletons -->
		{#if loading}
			{#each Array(skeletonCount) as _}
				<PostCardSkeleton />
			{/each}
		{/if}
	</div>

	<!-- End message -->
	{#if !hasMore && !loading && posts.length > 0}
		<div class="dark:text-mofu-dark-300 pb-20 text-center text-lg font-bold">모든 포스트를 확인하셨습니다 ✨</div>
	{:else if posts.length > 8 && !loading && hasMore}
		<div class="dark:text-mofu-dark-300 pb-20 text-center text-lg font-bold">
			스크롤하여 더 많은 포스트를 확인하세요 📜
		</div>
	{/if}

	<!-- Empty state -->
	{#if !initialLoading && !loading && posts.length === 0}
		<div class="flex flex-col items-center justify-center py-12 text-center">
			{#if selectedTags.length > 0}
				<div class="text-mofu-dark-400 mb-2 text-lg">선택한 태그와 일치하는 포스트가 없습니다</div>
				<div class="text-mofu-dark-500 text-sm mb-4">
					다른 태그를 선택하거나 필터를 해제해보세요.
				</div>
				<button 
					onclick={() => selectedTags = []}
					class="text-mofu hover:text-mofu/80 text-sm underline"
				>
					모든 필터 해제
				</button>
			{:else if allPosts.length === 0}
				<div class="text-mofu-dark-400 mb-2 text-lg">작성된 포스트가 없습니다</div>
				<div class="text-mofu-dark-500 text-sm">
					{profile.name}님이 아직 포스트를 작성하지 않았습니다.
				</div>
			{/if}
		</div>
	{/if}
</div>

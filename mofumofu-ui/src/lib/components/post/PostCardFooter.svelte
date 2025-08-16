<script lang="ts">
	import { Heart, Eye, Icon } from 'svelte-hero-icons';
	import { createLike, deleteLike, checkLikeStatus } from '$lib/api/like/likeApi';
	import { authStore } from '$lib/stores/auth.svelte';
	import { goto } from '$app/navigation';
	import * as m from '../../../paraglide/messages';

	const {
		author_name = '',
		author_avatar = '',
		author_handle = '',
		post_slug = '',
		likes = 0,
		views = 0,
		isSkeleton = false
	}: {
		author_name: string;
		author_avatar: string;
		author_handle: string;
		post_slug: string;
		likes: number;
		views: number;
		isSkeleton?: boolean;
	} = $props();

	// Like 관련 상태
	let isLiked = $state(false);
	let likeCount = $state(likes);
	let isLikeLoading = $state(true);
	let isLikeSubmitting = $state(false);

	// Like 상태 확인
	async function loadLikeStatus() {
		if (!authStore.isAuthenticated || isSkeleton) {
			isLikeLoading = false;
			return;
		}

		try {
			const response = await checkLikeStatus({
				handle: author_handle,
				slug: post_slug
			});
			isLiked = response.is_liked;
		} catch (error) {
			console.error('Failed to check like status:', error);
		} finally {
			isLikeLoading = false;
		}
	}

	// Like 토글
	async function toggleLike(e: Event) {
		e.preventDefault();
		e.stopPropagation();

		if (!authStore.isAuthenticated) {
			goto('/account/signin');
			return;
		}

		if (isLikeSubmitting) return;

		isLikeSubmitting = true;
		try {
			if (isLiked) {
				await deleteLike({
					handle: author_handle,
					slug: post_slug
				});
				isLiked = false;
				likeCount = Math.max(0, likeCount - 1);
			} else {
				await createLike({
					handle: author_handle,
					slug: post_slug
				});
				isLiked = true;
				likeCount += 1;
			}
		} catch (error) {
			console.error('Failed to toggle like:', error);
		} finally {
			isLikeSubmitting = false;
		}
	}

	// 컴포넌트 마운트 시 like 상태 로드
	$effect(() => {
		loadLikeStatus();
	});
</script>

<div class="flex items-center justify-between px-3 py-2">
	{#if isSkeleton}
		<div class="flex items-center gap-1.5">
			<div class="shimmer h-5 w-5 rounded-full"></div>
			<div class="shimmer h-3 w-14 rounded"></div>
		</div>
		<div class="flex items-center gap-3">
			<div class="shimmer h-3 w-6 rounded"></div>
			<div class="shimmer h-3 w-6 rounded"></div>
		</div>
	{:else}
		<a href="/@{author_handle}/profile" class="flex items-center gap-1.5 transition-opacity hover:opacity-80">
			<img src={author_avatar} alt={author_name} class="h-6 w-6 rounded-full object-cover" />
			<div class="gap-1">
				<span class="dark:text-mofu-dark-300 truncate text-xs">by</span>
				<span class="text-xs font-semibold">{author_name}</span>
			</div>
		</a>
		<div class="dark:text-mofu-dark-200 flex items-center gap-3 text-xs">
			<div class="flex items-center gap-1">
				<Icon src={Eye} solid size="16" class="text-mofu-dark-200" />
				<span>{views}</span>
			</div>

			{#if isLikeLoading}
				<div class="flex items-center gap-1">
					<div class="h-4 w-4 animate-pulse rounded bg-gray-300 dark:bg-gray-600"></div>
					<span>{likeCount}</span>
				</div>
			{:else}
				<button
					onclick={toggleLike}
					disabled={isLikeSubmitting}
					class="flex items-center gap-1 transition-colors {isLiked
						? 'text-rose-600 dark:text-rose-500'
						: 'hover:text-rose-600 dark:hover:text-rose-500'} 
					{isLikeSubmitting ? 'cursor-not-allowed opacity-50' : ''}"
				>
					<Icon src={Heart} size="16" solid />
					<span>{likeCount}</span>
				</button>
			{/if}
		</div>
	{/if}
</div>

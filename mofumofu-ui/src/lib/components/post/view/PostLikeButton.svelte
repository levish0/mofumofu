<script lang="ts">
	import { Heart, Icon } from 'svelte-hero-icons';
	import { createLike, deleteLike, checkLikeStatus } from '$lib/api/like/likeApi';
	import { authStore } from '$lib/stores/auth.svelte';
	import { goto } from '$app/navigation';
	import { toast } from 'svelte-sonner';
	import { onMount } from 'svelte';

	interface Props {
		postId: string;
		initialLikeCount: number;
	}

	const { postId, initialLikeCount }: Props = $props();

	let isLiked = $state(false);
	let likeCount = $state(initialLikeCount || 0);
	let isLikeLoading = $state(true);
	let isLikeSubmitting = $state(false);

	async function loadLikeStatus() {
		if (!authStore.isAuthenticated) {
			isLikeLoading = false;
			return;
		}

		try {
			const response = await checkLikeStatus({ post_id: postId });
			isLiked = response.is_liked;
		} catch (error) {
			console.error('Failed to check like status:', error);
		} finally {
			isLikeLoading = false;
		}
	}

	async function toggleLike() {
		if (!authStore.isAuthenticated) {
			goto('/account/signin');
			return;
		}

		if (isLikeSubmitting) return;

		isLikeSubmitting = true;
		try {
			if (isLiked) {
				await deleteLike({ post_id: postId });
				isLiked = false;
				likeCount = Math.max(0, likeCount - 1);
			} else {
				await createLike({ post_id: postId });
				isLiked = true;
				likeCount += 1;
			}
		} catch (error) {
			console.error('Failed to toggle like:', error);
			toast.error('좋아요 처리에 실패했습니다');
		} finally {
			isLikeSubmitting = false;
		}
	}

	onMount(() => {
		loadLikeStatus();
	});
</script>

{#if isLikeLoading}
	<div class="shimmer h-6 w-10 rounded"></div>
{:else}
	<button
		onclick={toggleLike}
		disabled={isLikeSubmitting}
		class="flex items-center gap-2 rounded-full px-4 py-2 transition-colors {isLiked
			? 'text-rose-600 dark:text-rose-500'
			: 'dark:text-mofu-dark-400 text-mofu-light-400 hover:text-rose-600 dark:hover:text-rose-500'} 
		{isLikeSubmitting ? 'cursor-not-allowed opacity-50' : ''}"
	>
		<Icon src={Heart} class="h-5 w-5" solid />
		<span class="text-sm">{likeCount}</span>
	</button>
{/if}

<script lang="ts">
	import { Heart, Icon } from 'svelte-hero-icons';

	interface Props {
		likeCount: number;
		userLiked: boolean;
		isLiking: boolean;
		isDeleted: boolean;
		isLoggedIn: boolean;
		onLike: () => void;
	}

	let { likeCount, userLiked, isLiking, isDeleted, isLoggedIn, onLike }: Props = $props();
</script>

{#if isLoggedIn}
	<button
		onclick={(e) => {
			e.stopPropagation();
			onLike();
		}}
		disabled={isLiking || isDeleted}
		class="flex items-center gap-2 rounded-full px-4 py-2 transition-colors {userLiked
			? 'text-rose-600 dark:text-rose-500'
			: 'dark:text-mofu-dark-400 text-mofu-light-400 hover:text-rose-600 dark:hover:text-rose-500'} 
		{isLiking ? 'cursor-not-allowed opacity-50' : ''}"
	>
		<Icon src={Heart} class="h-5 w-5" solid />
		{#if likeCount > 0}
			<span class="text-sm">{likeCount}</span>
		{/if}
		{#if isLiking}
			<div class="border-mofu-light-400 dark:border-mofu-dark-400 h-3 w-3 animate-spin rounded-full border-2 border-t-transparent"></div>
		{/if}
	</button>
{:else if likeCount > 0}
	<div class="text-mofu-light-400 dark:text-mofu-dark-400 flex items-center gap-2 px-4 py-2 text-sm">
		<Icon src={Heart} class="h-5 w-5" solid />
		<span class="text-sm">{likeCount}</span>
	</div>
{/if}
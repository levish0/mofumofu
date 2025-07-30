<script lang="ts">
	import { Heart, Icon } from 'svelte-hero-icons';

	const {
		author_name = '',
		author_avatar = '',
		likes = 0,
		isSkeleton = false
	}: {
		author_name: string;
		author_avatar: string;
		likes: number;
		isSkeleton?: boolean;
	} = $props();
</script>

<div class="flex items-center justify-between px-3 py-2">
	{#if isSkeleton}
		<div class="flex items-center gap-1.5">
			<div class="shimmer h-5 w-5 rounded-full"></div>
			<div class="shimmer h-3 w-14 rounded"></div>
		</div>
		<div class="shimmer h-3 w-6 rounded"></div>
	{:else}
		<div class="flex items-center gap-1.5">
			<img src={author_avatar} alt={author_name} class="h-6 w-6 rounded-full object-cover" />
			<div class="gap-1">
			<span class="dark:text-mofu-dark-300 truncate text-xs">by</span> 
			<span class="text-xs font-semibold">{author_name}</span>
			</div>
		</div>
		<div class="dark:text-mofu-dark-200 flex items-center gap-1 text-xs">
			<Icon src={Heart} size="16" class="text-mofu-dark-200" solid />
			<span>{likes}</span>
		</div>
	{/if}
</div>

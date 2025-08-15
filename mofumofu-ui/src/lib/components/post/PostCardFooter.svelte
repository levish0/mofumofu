<script lang="ts">
	import { Heart, Eye, Icon } from 'svelte-hero-icons';

	const {
		author_name = '',
		author_avatar = '',
		author_handle = '',
		likes = 0,
		views = 0,
		isSkeleton = false
	}: {
		author_name: string;
		author_avatar: string;
		author_handle: string;
		likes: number;
		views: number;
		isSkeleton?: boolean;
	} = $props();
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
			<div class="flex items-center gap-1">
				<Icon src={Heart} size="16" class="text-mofu-dark-200" solid />
				<span>{likes}</span>
			</div>
		</div>
	{/if}
</div>

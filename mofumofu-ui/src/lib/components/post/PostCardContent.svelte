<script lang="ts">
	const {
		title = '',
		summary = '',
		date = '',
		comments = 0,
		views = '',
		isSkeleton = false
	}: {
		title: string;
		summary: string;
		date: string;
		comments: number;
		views: string;
		isSkeleton?: boolean;
	} = $props();
</script>

<div class="flex flex-1 flex-col px-3 py-2">
	{#if isSkeleton}
		<div class="shimmer mb-3 h-5 w-4/5 rounded"></div>
		<div class="mb-4 space-y-2">
			<div class="shimmer h-3 w-full rounded"></div>
			<div class="shimmer h-3 w-full rounded"></div>
			<div class="shimmer h-3 w-3/4 rounded"></div>
		</div>

		<div class="mt-auto py-2">
			<div class="flex items-center gap-2 text-xs">
				<div class="shimmer h-3 w-20 rounded"></div>
				<div class="shimmer h-3 w-16 rounded"></div>
			</div>
		</div>
	{:else}
		<h3 class="text-mofu-dark-100 mb-3 line-clamp-1 text-lg font-bold">{title}</h3>
		<p class="text-mofu-dark-300 mb-4 line-clamp-3 text-sm">{summary}</p>

		<div class="mt-auto py-2">
			<div class="text-mofu-dark-300 flex items-center text-xs">
				<span>{date}</span>
				<span class="mx-1">·</span>
				<span>{comments}개의 댓글</span>
			</div>
		</div>
	{/if}
</div>

<script lang="ts">
	const {
		image = undefined,
		title = '',
		isSkeleton = false
	}: {
		image?: string;
		title: string;
		isSkeleton?: boolean;
	} = $props();

	let imageLoaded = $state(false);

	$effect(() => {
		if (image) {
			imageLoaded = false;
		}
	});
</script>

<div class="relative w-full overflow-hidden pt-[56.25%]">
	{#if isSkeleton}
		<div class="shimmer absolute inset-0"></div>
	{:else if image}
		{#if !imageLoaded}
			<div class="shimmer absolute inset-0"></div>
		{/if}

		<img
			src={image}
			alt={title}
			loading="lazy"
			class="absolute inset-0 h-full w-full object-cover transition-opacity duration-300"
			class:opacity-0={!imageLoaded}
			class:opacity-100={imageLoaded}
			onload={() => (imageLoaded = true)}
			onerror={() => (imageLoaded = true)}
		/>
	{/if}
</div>

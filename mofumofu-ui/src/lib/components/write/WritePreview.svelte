<script lang="ts">
	interface Props {
		title: string;
		tags: string;
		htmlOutput: string;
	}

	const { title, tags, htmlOutput }: Props = $props();
</script>

<div class="bg-mofu-dark-950 flex h-full flex-col pt-20 pl-8">
	<div class="mb-6">
		<h1 class="text-4xl font-bold text-white" style="font-size: 2.5rem; line-height: 1.2;">
			{title}
		</h1>
	</div>

	{#if tags}
		<div class="mb-8">
			<div class="flex flex-wrap gap-2">
				{#each tags.split(',') as tag}
					<span class="rounded bg-gray-700 px-2 py-1 text-sm text-gray-300">
						{tag.trim()}
					</span>
				{/each}
			</div>
		</div>
	{/if}

	<div class="flex-1 overflow-auto pr-4 pb-4">
		<div class="prose prose-invert prose-lg max-w-none text-white">
			{@html htmlOutput}
		</div>
	</div>
</div>

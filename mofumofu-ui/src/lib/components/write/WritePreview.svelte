<script lang="ts">
	interface Props {
		title: string;
		htmlOutput: string;
	}

	const { title, htmlOutput }: Props = $props();
</script>

<div class="bg-mofu-dark-950 dark:text-mofu-dark-200 flex h-full flex-col pt-20 pl-8">
	<div class="mb-4">
		<h1 class="text-4xl font-bold" style="font-size: 2.5rem; line-height: 1.2;">
			{title}
		</h1>
	</div>

	<div class="flex-1 overflow-auto pt-4 pr-4 pb-4">
		<div class="prose prose-invert text-mofu-dark-200 prose-lg max-w-none">
			{@html htmlOutput}
		</div>
	</div>
</div>

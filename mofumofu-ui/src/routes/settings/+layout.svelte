<script lang="ts">
	let { children } = $props();
</script>

<div class="mx-auto max-w-6xl pb-10">
	<div class="flex flex-col gap-5 px-4 pt-2 lg:flex-row">{@render children()}</div>
</div>

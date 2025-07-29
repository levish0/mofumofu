<script lang="ts">
	import Footer from '$lib/components/Footer.svelte';

	let { children } = $props();
</script>

<div class="bg-mofu-dark-900 h-full">{@render children()}</div>

<Footer />

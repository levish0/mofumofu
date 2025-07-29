<script lang="ts">
	import Footer from '$lib/components/Footer.svelte';

	let { children } = $props();
</script>

<div class="bg-mofu-dark-900 h-screen ">{@render children()}</div>

<Footer />

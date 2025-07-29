<script lang="ts">
	import Navbar from '$lib/components/navbar/Navbar.svelte';

	let { children } = $props();
</script>

<div class="bg-mofu-dark-900 min-h-screen ">
<Navbar />
<main class="pt-15">
	{@render children()}
</main>
</div>

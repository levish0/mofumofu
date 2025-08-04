<script lang="ts">
	import '../app.css';
	import { ModeWatcher } from 'mode-watcher';
	import Footer from '$lib/components/Footer.svelte';

	let { children } = $props();
</script>

<ModeWatcher defaultMode="dark" />
<div class="dark:bg-mofu-dark-900 font-pretendard min-h-screen max-w-screen">
	<main>
		{@render children()}
	</main>
</div>

<script lang="ts">
	import '../app.css';
	import { ModeWatcher } from 'mode-watcher';
	import Footer from '$lib/components/Footer.svelte';
	import Navbar from '$lib/components/navbar/Navbar.svelte';
	import { useNavbarScroll } from '$lib/hooks/useNavbarScroll.svelte';
	import { setContext } from 'svelte';

	let { children } = $props();

	const { isVisible, isAtTop } = useNavbarScroll({
		navbarHeight: 60,
		scrollThreshold: 10
	});

	setContext('navbar', { isVisible, isAtTop });
</script>

<ModeWatcher defaultMode="dark" />
<div class="dark:bg-mofu-dark-900 font-pretendard min-h-screen max-w-screen">
	<Navbar {isVisible} {isAtTop} />
	<main class="pt-[60px]">
		{@render children()}
	</main>
	<Footer />
</div>

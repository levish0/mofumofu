<script lang="ts">
	import Navbar from '$lib/components/navbar/Navbar.svelte';
	import SearchPanel from '$lib/components/search/SearchPanel.svelte';
	import { useNavbarScroll } from '$lib/hooks/ui/useNavbarScroll.svelte';
	import { getContext, setContext } from 'svelte';

	let { children } = $props();

	const { isVisible, isAtTop } = useNavbarScroll({
		navbarHeight: 60,
		scrollThreshold: 10
	});

	setContext('navbar', { isVisible, isAtTop });
</script>

<div class="max-w-8xl mx-auto">
	<Navbar {isVisible} {isAtTop} />
	<div class="pt-[60px]">
		{@render children()}
	</div>
</div>

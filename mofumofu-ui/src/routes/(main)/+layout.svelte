<script lang="ts">
	import Footer from '$lib/components/Footer.svelte';
	import Navbar from '$lib/components/navbar/Navbar.svelte';
	import SearchPanel from '$lib/components/search/SearchPanel.svelte';
	import { useNavbarScroll } from '$lib/hooks/useNavbarScroll.svelte';

	let { children } = $props();

	const { isVisible, isAtTop } = useNavbarScroll({
		navbarHeight: 60,
		scrollThreshold: 10
	});
</script>

<div class=" min-h-screen">
	<Navbar {isVisible} {isAtTop} />
	<main class="max-w-8xl mx-auto pt-[60px]">
		<div class="flex flex-col gap-5 px-4 pt-2 lg:flex-row">
			<div class="lg:w-4/5">
				{@render children()}
			</div>
			<div class="lg:w-1/5">
				<SearchPanel {isVisible} {isAtTop} />
			</div>
		</div>
	</main>
	<Footer />
</div>

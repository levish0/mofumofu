<script lang="ts">
	import SearchPanel from '$lib/components/search/SearchPanel.svelte';
	import { getContext } from 'svelte';

	let { children } = $props();

	type NavbarContext = {
		isVisible: () => boolean;
		isAtTop: () => boolean;
	};

	const navbar = getContext<NavbarContext>('navbar');
</script>

<div class="max-w-8xl mx-auto">
	<div class="flex flex-col gap-5 px-4 pt-2 lg:flex-row">
		<div class="lg:w-4/5">
			{@render children()}
		</div>
		<div class="lg:w-1/5">
			<SearchPanel isVisible={navbar.isVisible} isAtTop={navbar.isAtTop} />
		</div>
	</div>
</div>

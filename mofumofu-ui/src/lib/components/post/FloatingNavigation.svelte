<script lang="ts">
	import { ChevronUp, ChevronDown } from 'svelte-hero-icons';
	import { Icon } from 'svelte-hero-icons';
	import { onMount } from 'svelte';

	let showScrollToTop = $state(false);
	let isScrolling = $state(false);

	function scrollToTop() {
		if (isScrolling) return;
		isScrolling = true;
		window.scrollTo({ top: 0, behavior: 'smooth' });
		setTimeout(() => {
			isScrolling = false;
		}, 800);
	}

	function scrollToBottom() {
		if (isScrolling) return;
		isScrolling = true;
		window.scrollTo({ top: document.body.scrollHeight, behavior: 'smooth' });
		setTimeout(() => {
			isScrolling = false;
		}, 800);
	}

	onMount(() => {
		const handleScroll = () => {
			showScrollToTop = window.scrollY > 300;
		};

		window.addEventListener('scroll', handleScroll);
		return () => window.removeEventListener('scroll', handleScroll);
	});
</script>

<!-- 플로팅 네비게이션 버튼들 (모바일에서만 표시) -->
<div class="fixed bottom-4 right-4 z-40 flex flex-col gap-2 md:hidden">
	<!-- 위로 가기 버튼 -->
	<button
		onclick={scrollToTop}
		disabled={isScrolling}
		class="flex h-12 w-12 items-center justify-center rounded-full bg-white shadow-lg transition-all duration-300 hover:scale-105 hover:shadow-xl disabled:opacity-50 dark:bg-gray-800"
		aria-label="위로 가기"
	>
		<Icon src={ChevronUp} class="h-6 w-6 text-gray-700 dark:text-gray-300" />
	</button>

	<!-- 아래로 가기 버튼 -->
	<button
		onclick={scrollToBottom}
		disabled={isScrolling}
		class="flex h-12 w-12 items-center justify-center rounded-full bg-white shadow-lg transition-all duration-300 hover:scale-105 hover:shadow-xl disabled:opacity-50 dark:bg-gray-800"
		aria-label="아래로 가기"
	>
		<Icon src={ChevronDown} class="h-6 w-6 text-gray-700 dark:text-gray-300" />
	</button>
</div>
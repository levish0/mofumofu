<script lang="ts">
	import { Bars3, XMark } from 'svelte-hero-icons';
	import { Icon } from 'svelte-hero-icons';
	import { fly, fade } from 'svelte/transition';

	type TOCItem = {
		id: string;
		text: string;
		level: number;
	};

	type Props = {
		tocItems: TOCItem[];
	};

	const { tocItems }: Props = $props();
	let isOpen = $state(false);

	function toggleTOC() {
		isOpen = !isOpen;
	}

	function closeTOC() {
		isOpen = false;
	}

	function handleTOCClick() {
		closeTOC();
	}
</script>

<!-- 플로팅 목차 버튼 (모바일에서만 표시) -->
<div class="fixed bottom-32 right-4 z-40 md:hidden">
	<!-- TOC 버튼 -->
	<button
		onclick={toggleTOC}
		class="flex h-12 w-12 items-center justify-center rounded-full bg-white shadow-lg transition-all duration-300 hover:scale-105 hover:shadow-xl dark:bg-gray-800"
		aria-label="목차 보기"
	>
		<Icon src={Bars3} class="h-6 w-6 text-gray-700 dark:text-gray-300" />
	</button>

	<!-- TOC 오버레이 -->
	{#if isOpen}
		<div
			class="fixed inset-0 z-40 bg-black/30 backdrop-blur-sm"
			transition:fade={{ duration: 200 }}
			onclick={closeTOC}
		></div>

		<!-- TOC 패널 -->
		<div
			class="fixed bottom-0 left-0 right-0 z-50 max-h-[70vh] overflow-hidden rounded-t-xl bg-white shadow-2xl dark:bg-gray-800"
			transition:fly={{ y: 300, duration: 300 }}
		>
			<!-- 헤더 -->
			<div class="flex items-center justify-between border-b p-4 dark:border-gray-700">
				<h3 class="text-lg font-semibold text-gray-900 dark:text-white">목차</h3>
				<button
					onclick={closeTOC}
					class="flex h-8 w-8 items-center justify-center rounded-full transition-colors hover:bg-gray-100 dark:hover:bg-gray-700"
				>
					<Icon src={XMark} class="h-5 w-5 text-gray-500 dark:text-gray-400" />
				</button>
			</div>

			<!-- 목차 내용 -->
			<div class="max-h-[50vh] overflow-y-auto p-4">
				<nav class="space-y-2">
					{#each tocItems as item}
						<a
							href="#{item.id}"
							onclick={handleTOCClick}
							class="block py-2 text-sm text-gray-600 transition-colors hover:text-gray-900 dark:text-gray-300 dark:hover:text-white"
							style="padding-left: {(item.level - 1) * 16}px"
						>
							{item.text}
						</a>
					{/each}
				</nav>
			</div>
		</div>
	{/if}
</div>
<!-- src/lib/components/search/SearchPanel.svelte -->
<script lang="ts">
	let { isVisible, isAtTop } = $props();

	let keyword = $state('');
	let tags = ['React', 'TypeScript', 'Next.js', 'Svelte', 'Zustand', 'UX'];
	let selected = $state<string[]>([]);

	// Calculate the top position based on navbar state
	const topPosition = $derived(isVisible() ? '68px' : '8px');
</script>

<div class="sticky w-full space-y-6 transition-all duration-100 ease-out" style="top: {topPosition}">
	<h2 class="text-xl font-bold text-white">검색 & 필터</h2>

	<!-- 검색창 -->
	<div>
		<label for="search" class="sr-only">검색어</label>
		<input
			id="search"
			type="text"
			bind:value={keyword}
			placeholder="제목 / 요약 검색"
			class="border-mofu-dark-600 bg-mofu-dark-700 w-full rounded-sm
		              px-3 py-2 text-sm text-white
		              placeholder-mofu-dark-300 outline-none"
		/>
	</div>

	<!-- 태그 필터 -->
	<div>
		<h3 class="mb-2 text-sm font-semibold text-gray-300">인기 태그</h3>
		<div class="flex flex-wrap gap-2">
			{#each tags as tag}
				<button
					onclick={() => {
						selected = selected.includes(tag) ? selected.filter((t) => t !== tag) : [...selected, tag];
					}}
					class="rounded-full px-3 py-1 text-xs transition-colors
					       {selected.includes(tag) ? 'bg-blue-600 text-white' : 'bg-mofu-dark-800 hover:bg-mofu-dark-500 text-gray-200'}"
				>
					{tag}
				</button>
			{/each}
		</div>
	</div>

	<!-- 정렬 -->
	<div>
		<h3 class="mb-2 text-sm font-semibold text-gray-300">정렬</h3>
		<select
			class="border-mofu-dark-600 bg-mofu-dark-700 w-full rounded-sm border
			       px-3 py-2 text-sm text-white focus:border-blue-500 focus:outline-none"
		>
			<option value="recent">최신순</option>
			<option value="popular">인기순 (좋아요)</option>
			<option value="comments">댓글 많은 순</option>
		</select>
	</div>
</div>

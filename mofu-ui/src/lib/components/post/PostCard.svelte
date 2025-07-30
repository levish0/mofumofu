<!-- src/components/post/PostCard.svelte  -->
<script lang="ts">
	export let image: string | undefined;
	export let title: string;
	export let summary: string;
	export let date: string;
	export let comments: number;
	export let views: string;
	export let author_name: string;
	export let author_avatar: string;
	export let likes: number;
</script>

<div
	class=" bg-mofu-dark-800 border-mofu-dark-800 group flex h-full cursor-pointer flex-col overflow-hidden rounded-sm border transition-all duration-200 hover:-translate-y-1 hover:shadow-xl"
>
	<!-- 이미지 섹션 (이미지가 있을 때만 표시) -->
	{#if image}
		<div class="h-32 w-full">
			<img src={image} alt={title} class="h-full w-full object-cover" />
		</div>
	{/if}

	<!-- 콘텐츠 섹션 -->
	<div class="flex flex-1 flex-col">
		<!-- 제목 -->
		<div class="px-4 pt-2">
			<h3 class="mb-3 line-clamp-1 text-lg leading-tight font-bold text-white">
				{title}
			</h3>

			<!-- 요약 -->
			<p class="mb-4 line-clamp-3 flex-1 text-sm leading-relaxed text-gray-400">
				{summary}
			</p>
		</div>
		<!-- 메타 정보 - 두 줄로 분리 -->
		<div class=" mt-auto py-2">
			<!-- 첫 번째 줄: 날짜와 댓글 개수 -->
			<div class="flex items-center px-4 py-2 text-xs text-gray-500">
				<span class="truncate">{date}</span>
				<span class="mx-1 flex-shrink-0">·</span>
				<span class="truncate">{comments}개의 {views}</span>
			</div>
			<div class="border-mofu-dark-600 space-y-2 border-t px-4 py-1">
				<!-- 두 번째 줄: 작성자와 좋아요 -->
				<div class="flex items-center justify-between">
					<!-- 작성자 -->
					<div class="flex min-w-0 flex-1 items-center gap-1.5">
						<img src={author_avatar} alt={author_name} class="h-5 w-5 flex-shrink-0 rounded-full object-cover" />
						<span class="truncate text-xs font-medium text-gray-500">by {author_name}</span>
					</div>

					<!-- 좋아요 -->
					<div class="ml-2 flex flex-shrink-0 items-center gap-1 text-gray-500">
						<svg class="h-3.5 w-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"
							></path>
						</svg>
						<span class="text-xs">{likes}</span>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>

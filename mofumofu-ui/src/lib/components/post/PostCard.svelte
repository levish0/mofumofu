<script lang="ts">
	import PostCardImage from './PostCardImage.svelte';
	import PostCardContent from './PostCardContent.svelte';
	import PostCardFooter from './PostCardFooter.svelte';

	const {
		image = undefined,
		title = '',
		summary = '',
		date = '',
		comments = 0,
		views = '',
		author_name = '',
		author_avatar = '',
		likes = 0
	} = $props();

	// skeleton 모드인지 확인 (모든 props가 비어있으면 skeleton)
	let isSkeleton = $derived(!title && !summary && !date && !author_name);
</script>

<!-- 카드 전체 -->
<div
	class="dark:bg-mofu-dark-800 dark:border-mofu-dark-800 border-mofu-light-300 group flex h-full cursor-pointer flex-col overflow-hidden rounded-xl border bg-white transition-all duration-200 hover:-translate-y-1 hover:opacity-75 hover:shadow-lg md:min-h-[300px]"
>
	<!-- 이미지 영역 (스켈레톤이거나 이미지가 있을 때만) -->
	{#if isSkeleton || image}
		<PostCardImage {image} {title} {isSkeleton} />
	{/if}

	<!-- 텍스트 영역 -->
	<PostCardContent {title} {summary} {date} {comments} {views} {isSkeleton} />

	<!-- 구분선과 푸터 -->
	<div class="dark:border-mofu-dark-600 border-mofu-light-400 border-t">
		<PostCardFooter {author_name} {author_avatar} {likes} {isSkeleton} />
	</div>
</div>

<script lang="ts">
	import PostCardImage from './PostCardImage.svelte';
	import PostCardContent from './PostCardContent.svelte';
	import PostCardFooter from './PostCardFooter.svelte';
</script>

<div
	class="bg-mofu-dark-800 border-mofu-dark-800 group flex h-full cursor-pointer flex-col overflow-hidden rounded-xl border transition-all duration-200 hover:-translate-y-1 hover:shadow-xl"
>
	<PostCardImage title="" isSkeleton={true} />

	<PostCardContent title="" summary="" date="" comments={0} views="" isSkeleton={true} />

	<div class="border-mofu-dark-600 border-t">
		<PostCardFooter author_name="" author_avatar="" likes={0} isSkeleton={true} />
	</div>
</div>

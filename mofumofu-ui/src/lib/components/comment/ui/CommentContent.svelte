<script lang="ts">
	interface Props {
		content: string;
	}

	let { content }: Props = $props();
</script>

<div class="comment-content whitespace-pre-wrap break-words">
	{content}
</div>
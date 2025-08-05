<script lang="ts">
	import PostCard from '../post/PostCard.svelte';

	type Post = {
		id: string;
		title: string;
		content: string;
		image?: string;
		images?: string[];
		timestamp: string;
		comments: number;
		likes: number;
	};

	type Props = {
		profile: {
			name: string;
			profile_image?: string;
		};
	};

	const { profile }: Props = $props();

	const mockPosts: Post[] = [
		{
			id: '1',
			title: 'lorem what',
			content: 'lorem what',
			image: 'https://picsum.photos/500/300?random=1',
			timestamp: '2h',
			comments: 12,
			likes: 28
		},
		{
			id: '2',
			title: 'lorem what',
			content: 'lorem what',
			timestamp: '1d',
			comments: 8,
			likes: 42
		},
		{
			id: '3',
			title: 'lorem what',
			content: 'lorem what',
			images: [
				'https://picsum.photos/250/150?random=2',
				'https://picsum.photos/250/150?random=3'
			],
			timestamp: '3d',
			comments: 3,
			likes: 19
		}
	];
</script>

<div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
	{#each mockPosts as post (post.id)}
		<PostCard 
			image={post.image}
			title={post.title}
			summary={post.content}
			date={post.timestamp}
			comments={post.comments}
			likes={post.likes}
			author_name={profile.name}
			author_avatar={profile.profile_image}
		/>
	{/each}
</div>
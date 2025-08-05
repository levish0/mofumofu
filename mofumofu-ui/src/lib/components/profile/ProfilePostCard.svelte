<script lang="ts">
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
		post: Post;
		profile: {
			name: string;
			profile_image?: string;
		};
	};

	const { post, profile }: Props = $props();
</script>

<div
	class="group flex h-full cursor-pointer flex-col overflow-hidden rounded-xl border border-gray-200 bg-white transition-all duration-200 hover:-translate-y-1 hover:opacity-75 hover:shadow-xl dark:border-gray-800 dark:bg-gray-900"
>
	<!-- Images -->
	{#if post.image}
		<div class="aspect-video w-full overflow-hidden">
			<img
				src={post.image}
				alt="Post image"
				class="h-full w-full object-cover transition-transform duration-200 group-hover:scale-105"
			/>
		</div>
	{:else if post.images && post.images.length > 0}
		<div class="grid grid-cols-2 gap-1">
			{#each post.images as image}
				<img
					src={image}
					alt="Post image"
					class="aspect-video w-full object-cover transition-transform duration-200 group-hover:scale-105"
				/>
			{/each}
		</div>
	{/if}

	<!-- Content -->
	<div class="flex flex-1 flex-col p-4">
		<h3 class="mb-2 line-clamp-2 text-lg font-semibold text-gray-900 dark:text-white">{post.title}</h3>
		<p class="mb-4 line-clamp-3 text-sm text-gray-600 dark:text-gray-300">{post.content}</p>
		<div class="mt-auto flex items-center justify-between text-sm text-gray-500 dark:text-gray-400">
			<span>{post.timestamp}</span>
			<div class="flex items-center gap-4">
				<span>{post.comments} comments</span>
				<span>{post.likes} likes</span>
			</div>
		</div>
	</div>

	<!-- Footer -->
	<div class="border-t border-gray-100 p-4 dark:border-gray-800">
		<div class="flex items-center justify-between">
			<div class="flex items-center gap-2">
				{#if profile.profile_image}
					<img
						src={profile.profile_image}
						alt={profile.name}
						class="h-8 w-8 rounded-full object-cover"
					/>
				{:else}
					<div class="flex h-8 w-8 items-center justify-center rounded-full bg-gray-100 dark:bg-gray-800">
						<span class="text-xs font-medium text-gray-600 dark:text-gray-400">
							{profile.name.charAt(0).toUpperCase()}
						</span>
					</div>
				{/if}
				<span class="text-sm font-medium text-gray-900 dark:text-white">{profile.name}</span>
			</div>
			<div class="flex items-center gap-1 text-red-500">
				<svg class="h-4 w-4" fill="currentColor" viewBox="0 0 24 24">
					<path
						d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z"
					/>
				</svg>
				<span class="text-sm font-medium">{post.likes}</span>
			</div>
		</div>
	</div>
</div>
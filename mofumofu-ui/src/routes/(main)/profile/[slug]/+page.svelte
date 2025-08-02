<script lang="ts">
	import type { PageData } from './$types';
	import { authStore } from '$lib/stores/auth.svelte';

	const { data }: { data: PageData } = $props();

	let activeTab = $state('posts');

	function handleEditProfile() {
		console.log('Edit profile clicked');
	}

	function handleFollow() {
		console.log('Follow clicked');
	}

	function setActiveTab(tab: string) {
		activeTab = tab;
	}
</script>

<svelte:head>
	<title>{data.profile.name} (@{data.profile.handle}) - Mofu</title>
</svelte:head>

<div class="min-h-screen bg-white dark:bg-gray-900">
	<!-- Banner -->
	<div class="relative aspect-[4/1] bg-gray-100 dark:bg-gray-800">
		{#if data.profile.banner_image}
			<img src={data.profile.banner_image} alt="Banner" class="h-full w-full object-cover" />
		{/if}
	</div>

	<div class="mx-auto max-w-4xl">
		<!-- Profile Header -->
		<div class="relative z-10 border-b border-gray-100 bg-white dark:border-gray-800 dark:bg-gray-900">
			<div class="-mt-12 px-6 pt-6 pb-6">
				<div class="flex items-start space-x-6">
					<!-- Profile Image -->
					{#if data.profile.profile_image}
						<img
							src={data.profile.profile_image}
							alt={data.profile.name}
							class="relative z-10 h-24 w-24 rounded-full border-4 border-white object-cover dark:border-gray-900"
						/>
					{:else}
						<div
							class="relative z-10 flex h-24 w-24 items-center justify-center rounded-full border-4 border-white bg-white dark:border-gray-900 dark:bg-gray-900"
						>
							<span class="text-2xl font-medium text-gray-600 dark:text-gray-400">
								{data.profile.name.charAt(0).toUpperCase()}
							</span>
						</div>
					{/if}

					<!-- Profile Info -->
					<div class="mt-12 min-w-0 flex-1">
						<div class="flex items-center justify-between">
							<div>
								<h1 class="text-2xl font-semibold text-gray-900 dark:text-white">
									{data.profile.name}
								</h1>
								<p class="mt-1 text-gray-500 dark:text-gray-400">
									@{data.profile.handle}
								</p>
								{#if data.profile.bio}
									<p class="mt-3 text-gray-700 dark:text-gray-300">
										{data.profile.bio}
									</p>
								{/if}
							</div>

							<!-- Action Button -->
							<div class="flex-shrink-0">
								{#if data.isOwnProfile}
									<button
										onclick={handleEditProfile}
										class="rounded-lg border border-gray-300 px-4 py-2 text-sm font-medium text-gray-700 transition-colors hover:bg-gray-50 dark:border-gray-600 dark:text-gray-300 dark:hover:bg-gray-800"
									>
										Edit Profile
									</button>
								{:else if authStore.isAuthenticated}
									<button
										onclick={handleFollow}
										class="rounded-lg bg-black px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-gray-800 dark:bg-white dark:text-black dark:hover:bg-gray-100"
									>
										Follow
									</button>
								{/if}
							</div>
						</div>

						<!-- Stats -->
						<div class="mt-4 flex items-center space-x-6">
							<div class="text-sm">
								<span class="font-semibold text-gray-900 dark:text-white">0</span>
								<span class="ml-1 text-gray-500 dark:text-gray-400">Following</span>
							</div>
							<div class="text-sm">
								<span class="font-semibold text-gray-900 dark:text-white">0</span>
								<span class="ml-1 text-gray-500 dark:text-gray-400">Followers</span>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>

		<!-- Navigation Tabs -->
		<div class="border-b border-gray-100 dark:border-gray-800">
			<nav class="flex">
				<button
					onclick={() => setActiveTab('posts')}
					class="border-b-2 px-6 py-4 text-sm font-medium transition-colors {activeTab === 'posts'
						? 'border-black text-black dark:border-white dark:text-white'
						: 'border-transparent text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'}"
				>
					Posts
				</button>
				<button
					onclick={() => setActiveTab('replies')}
					class="border-b-2 px-6 py-4 text-sm font-medium transition-colors {activeTab === 'replies'
						? 'border-black text-black dark:border-white dark:text-white'
						: 'border-transparent text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'}"
				>
					Replies
				</button>
				<button
					onclick={() => setActiveTab('media')}
					class="border-b-2 px-6 py-4 text-sm font-medium transition-colors {activeTab === 'media'
						? 'border-black text-black dark:border-white dark:text-white'
						: 'border-transparent text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'}"
				>
					Media
				</button>
				<button
					onclick={() => setActiveTab('likes')}
					class="border-b-2 px-6 py-4 text-sm font-medium transition-colors {activeTab === 'likes'
						? 'border-black text-black dark:border-white dark:text-white'
						: 'border-transparent text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'}"
				>
					Likes
				</button>
			</nav>
		</div>

		<!-- Content -->
		<div class="px-6 py-8">
			{#if activeTab === 'posts'}
				<div class="py-16 text-center">
					<div
						class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-gray-100 dark:bg-gray-800"
					>
						<svg class="h-8 w-8 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.746 0 3.332.477 4.5 1.253v13C19.832 18.477 18.246 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"
							></path>
						</svg>
					</div>
					<h3 class="mb-1 text-lg font-medium text-gray-900 dark:text-white">No posts yet</h3>
					<p class="text-gray-500 dark:text-gray-400">
						{#if data.isOwnProfile}
							When you post something, it'll show up here.
						{:else}
							{data.profile.name} hasn't posted anything yet.
						{/if}
					</p>
				</div>
			{:else if activeTab === 'replies'}
				<div class="py-16 text-center">
					<div
						class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-gray-100 dark:bg-gray-800"
					>
						<svg class="h-8 w-8 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M7 8h10M7 12h4m1 8l-4-4H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-3l-4 4z"
							></path>
						</svg>
					</div>
					<h3 class="mb-1 text-lg font-medium text-gray-900 dark:text-white">No replies yet</h3>
					<p class="text-gray-500 dark:text-gray-400">
						{#if data.isOwnProfile}
							When you reply to someone, it'll show up here.
						{:else}
							{data.profile.name} hasn't replied to anything yet.
						{/if}
					</p>
				</div>
			{:else if activeTab === 'media'}
				<div class="py-16 text-center">
					<div
						class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-gray-100 dark:bg-gray-800"
					>
						<svg class="h-8 w-8 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
							></path>
						</svg>
					</div>
					<h3 class="mb-1 text-lg font-medium text-gray-900 dark:text-white">No media yet</h3>
					<p class="text-gray-500 dark:text-gray-400">
						{#if data.isOwnProfile}
							Photos and videos you post will appear here.
						{:else}
							{data.profile.name} hasn't shared any media yet.
						{/if}
					</p>
				</div>
			{:else if activeTab === 'likes'}
				<div class="py-16 text-center">
					<div
						class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-gray-100 dark:bg-gray-800"
					>
						<svg class="h-8 w-8 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"
							></path>
						</svg>
					</div>
					<h3 class="mb-1 text-lg font-medium text-gray-900 dark:text-white">No likes yet</h3>
					<p class="text-gray-500 dark:text-gray-400">
						{#if data.isOwnProfile}
							Posts you like will appear here.
						{:else}
							{data.profile.name} hasn't liked anything yet.
						{/if}
					</p>
				</div>
			{/if}
		</div>
	</div>
</div>

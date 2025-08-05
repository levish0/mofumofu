<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { authStore } from '$lib/stores/auth.svelte.js';
	import { getContext } from 'svelte';
	import { getMyProfile } from '$lib/api/user/userApi';
	import type { PageData } from './$types';

	const { data }: { data: PageData } = $props();

	let isLoading = $state(true);
	let isOwnProfile = $state(false);
	let bannerLoaded = $state(false);
	let profileImageLoaded = $state(false);

	type NavbarContext = {
		isVisible: () => boolean;
		isAtTop: () => boolean;
	};

	const navbar = getContext<NavbarContext>('navbar');

	// Calculate the top position based on navbar state
	const topPosition = $derived(navbar.isVisible() ? '68px' : '8px');

	// Reactively check if this is the user's own profile when data changes
	$effect(() => {
		async function checkOwnership() {
			if (authStore.isAuthenticated && data.profile.handle) {
				isLoading = true;
				try {
					const currentUser = await getMyProfile();
					isOwnProfile = currentUser.handle === data.profile.handle;
				} catch (err) {
					console.warn('Failed to get current user profile:', err);
					isOwnProfile = false;
				}
			} else {
				isOwnProfile = false;
			}
			isLoading = false;
		}

		checkOwnership();
	});

	function handleEditProfile() {
		goto('/settings');
	}

	function handleFollow() {
		console.log('Follow user:', data.profile.handle);
	}
</script>

<svelte:head>
	<title>{data.profile.name} (@{data.profile.handle}) - Mofu</title>
</svelte:head>
<div class="min-h-screen">
	<div class="max-w-8xl mx-auto px-4 pt-2">
		<!-- Two Column Layout -->
		<div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
			<!-- Left Column: Profile Info (Twitter Style) -->
			<div class="lg:col-span-1">
				<div class="sticky transition-all duration-100 ease-out" style="top: {topPosition}">
					<!-- Banner Section -->
					<div class="relative aspect-[3/1] w-full">
						{#if data.profile.banner_image}
							{#if !bannerLoaded}
								<div class="shimmer bg-mofu-dark-900 absolute inset-0 overflow-hidden rounded-xl"></div>
							{/if}
							<img
								src={data.profile.banner_image}
								alt="Banner"
								class="absolute inset-0 h-full w-full overflow-hidden rounded-xl object-cover transition-opacity duration-300"
								class:opacity-0={!bannerLoaded}
								class:opacity-100={bannerLoaded}
								loading="lazy"
								onload={() => (bannerLoaded = true)}
								onerror={() => (bannerLoaded = true)}
							/>
						{:else}
							<div class="h-full w-full overflow-hidden rounded-xl bg-gradient-to-r from-blue-400 to-purple-500"></div>
						{/if}

						<!-- Action Button (next to profile image) -->
						<div class="absolute right-4 -bottom-12 z-20">
							{#if isLoading}
								<div class="shimmer h-10 w-20 rounded-full"></div>
							{:else if isOwnProfile}
								<button
									onclick={handleEditProfile}
									class="rounded-full border border-gray-300 px-4 py-2 text-sm font-medium text-gray-700 transition-colors hover:bg-gray-50 dark:border-gray-600 dark:text-gray-300 dark:hover:bg-gray-800"
								>
									Edit profile
								</button>
							{:else if authStore.isAuthenticated}
								<button
									onclick={handleFollow}
									class="rounded-full bg-black px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-gray-800 dark:bg-white dark:text-black dark:hover:bg-gray-100"
								>
									Follow
								</button>
							{/if}
						</div>

						<!-- Profile Image (overlapping banner) -->
						<div class="absolute -bottom-12 left-4 z-10">
							<div class="relative h-24 w-24">
								{#if data.profile.profile_image}
									{#if !profileImageLoaded}
										<div
											class="shimmer bg-mofu-dark-900 border-mofu-dark-900 absolute inset-0 rounded-full border-4"
										></div>
									{/if}
									<img
										src={data.profile.profile_image}
										alt={data.profile.name}
										class="border-mofu-dark-900 bg-mofu-dark-900 absolute inset-0 h-24 w-24 rounded-full border-4 object-cover transition-opacity duration-300"
										class:opacity-0={!profileImageLoaded}
										class:opacity-100={profileImageLoaded}
										loading="lazy"
										onload={() => (profileImageLoaded = true)}
										onerror={() => (profileImageLoaded = true)}
									/>
								{:else}
									<div
										class="flex h-24 w-24 items-center justify-center rounded-full border-4 border-white bg-gray-100 dark:border-gray-900 dark:bg-gray-800"
									>
										<span class="text-2xl font-medium text-gray-600 dark:text-gray-400">
											{data.profile.name.charAt(0).toUpperCase()}
										</span>
									</div>
								{/if}
							</div>
						</div>
					</div>

					<!-- Profile Content -->
					<div class="px-4 pt-14 pb-4">
						<!-- Profile Info -->
						<div class="space-y-2">
							<div>
								<h1 class="text-xl font-bold text-gray-900 dark:text-white">
									{data.profile.name}
								</h1>
								<p class="text-sm text-gray-500 dark:text-gray-400">
									@{data.profile.handle}
								</p>
							</div>

							<!-- Bio -->
							<p class="text-sm text-gray-700 dark:text-gray-300">
								✨ 아 파이썬으로 백엔드 짤걸<br />
								✨ 괜히 러스트로 짜가지고 <br />
								⚠️ 아 자고 싶다
							</p>

							<!-- Location and Join Date -->
							<div class="space-y-1 text-sm text-gray-500 dark:text-gray-400">
								<div class="flex items-center gap-1">
									<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"
										></path>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"
										></path>
									</svg>
									<span>tokyo</span>
								</div>
								<div class="flex items-center gap-1">
									<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M8 7V3a2 2 0 012-2h4a2 2 0 012 2v4m-6 4v10m6-10v10M6 21h12a2 2 0 002-2V9a2 2 0 00-2-2H6a2 2 0 00-2 2v10a2 2 0 002 2z"
										></path>
									</svg>
									<span>Joined August 2017</span>
								</div>
							</div>

							<!-- Stats -->
							<div class="flex items-center space-x-4 text-sm">
								<div>
									<span class="font-bold text-gray-900 dark:text-white">427</span>
									<span class="text-gray-500 dark:text-gray-400">Following</span>
								</div>
								<div>
									<span class="font-bold text-gray-900 dark:text-white">152.1K</span>
									<span class="text-gray-500 dark:text-gray-400">Followers</span>
								</div>
							</div>

							<!-- Additional Info -->
							<p class="text-xs text-gray-500 dark:text-gray-400">Not followed by anyone you're following</p>
						</div>
					</div>
				</div>
			</div>

			<!-- Right Column: Posts -->
			<div class="lg:col-span-1">
				<div class="space-y-6">
					<!-- Post Card 1 -->
					<div
						class="group flex h-full cursor-pointer flex-col overflow-hidden rounded-xl border border-gray-200 bg-white transition-all duration-200 hover:-translate-y-1 hover:opacity-75 hover:shadow-xl dark:border-gray-800 dark:bg-gray-900"
					>
						<!-- Image -->
						<div class="aspect-video w-full overflow-hidden">
							<img
								src="https://picsum.photos/500/300?random=1"
								alt="Post image"
								class="h-full w-full object-cover transition-transform duration-200 group-hover:scale-105"
							/>
						</div>

						<!-- Content -->
						<div class="flex flex-1 flex-col p-4">
							<h3 class="mb-2 line-clamp-2 text-lg font-semibold text-gray-900 dark:text-white">lorem what</h3>
							<p class="mb-4 line-clamp-3 text-sm text-gray-600 dark:text-gray-300">lorem what</p>
							<div class="mt-auto flex items-center justify-between text-sm text-gray-500 dark:text-gray-400">
								<span>2h</span>
								<div class="flex items-center gap-4">
									<span>12 comments</span>
									<span>28 likes</span>
								</div>
							</div>
						</div>

						<!-- Footer -->
						<div class="border-t border-gray-100 p-4 dark:border-gray-800">
							<div class="flex items-center justify-between">
								<div class="flex items-center gap-2">
									{#if data.profile.profile_image}
										<img
											src={data.profile.profile_image}
											alt={data.profile.name}
											class="h-8 w-8 rounded-full object-cover"
										/>
									{:else}
										<div class="flex h-8 w-8 items-center justify-center rounded-full bg-gray-100 dark:bg-gray-800">
											<span class="text-xs font-medium text-gray-600 dark:text-gray-400">
												{data.profile.name.charAt(0).toUpperCase()}
											</span>
										</div>
									{/if}
									<span class="text-sm font-medium text-gray-900 dark:text-white">{data.profile.name}</span>
								</div>
								<div class="flex items-center gap-1 text-red-500">
									<svg class="h-4 w-4" fill="currentColor" viewBox="0 0 24 24">
										<path
											d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z"
										/>
									</svg>
									<span class="text-sm font-medium">28</span>
								</div>
							</div>
						</div>
					</div>

					<!-- Post Card 2 -->
					<div
						class="group flex h-full cursor-pointer flex-col overflow-hidden rounded-xl border border-gray-200 bg-white transition-all duration-200 hover:-translate-y-1 hover:opacity-75 hover:shadow-xl dark:border-gray-800 dark:bg-gray-900"
					>
						<!-- Content -->
						<div class="flex flex-1 flex-col p-4">
							<h3 class="mb-2 line-clamp-2 text-lg font-semibold text-gray-900 dark:text-white">lorem what</h3>
							<p class="mb-4 line-clamp-3 text-sm text-gray-600 dark:text-gray-300">lorem what</p>
							<div class="mt-auto flex items-center justify-between text-sm text-gray-500 dark:text-gray-400">
								<span>1d</span>
								<div class="flex items-center gap-4">
									<span>8 comments</span>
									<span>42 likes</span>
								</div>
							</div>
						</div>

						<!-- Footer -->
						<div class="border-t border-gray-100 p-4 dark:border-gray-800">
							<div class="flex items-center justify-between">
								<div class="flex items-center gap-2">
									{#if data.profile.profile_image}
										<img
											src={data.profile.profile_image}
											alt={data.profile.name}
											class="h-8 w-8 rounded-full object-cover"
										/>
									{:else}
										<div class="flex h-8 w-8 items-center justify-center rounded-full bg-gray-100 dark:bg-gray-800">
											<span class="text-xs font-medium text-gray-600 dark:text-gray-400">
												{data.profile.name.charAt(0).toUpperCase()}
											</span>
										</div>
									{/if}
									<span class="text-sm font-medium text-gray-900 dark:text-white">{data.profile.name}</span>
								</div>
								<div class="flex items-center gap-1 text-red-500">
									<svg class="h-4 w-4" fill="currentColor" viewBox="0 0 24 24">
										<path
											d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z"
										/>
									</svg>
									<span class="text-sm font-medium">42</span>
								</div>
							</div>
						</div>
					</div>

					<!-- Post Card 3 -->
					<div
						class="group flex h-full cursor-pointer flex-col overflow-hidden rounded-xl border border-gray-200 bg-white transition-all duration-200 hover:-translate-y-1 hover:opacity-75 hover:shadow-xl dark:border-gray-800 dark:bg-gray-900"
					>
						<!-- Images Grid -->
						<div class="grid grid-cols-2 gap-1">
							<img
								src="https://picsum.photos/250/150?random=2"
								alt="Tailwind performance"
								class="aspect-video w-full object-cover transition-transform duration-200 group-hover:scale-105"
							/>
							<img
								src="https://picsum.photos/250/150?random=3"
								alt="Build output"
								class="aspect-video w-full object-cover transition-transform duration-200 group-hover:scale-105"
							/>
						</div>

						<!-- Content -->
						<div class="flex flex-1 flex-col p-4">
							<h3 class="mb-2 line-clamp-2 text-lg font-semibold text-gray-900 dark:text-white">lorem what</h3>
							<p class="mb-4 line-clamp-3 text-sm text-gray-600 dark:text-gray-300">lorem what</p>
							<div class="mt-auto flex items-center justify-between text-sm text-gray-500 dark:text-gray-400">
								<span>3d</span>
								<div class="flex items-center gap-4">
									<span>3 comments</span>
									<span>19 likes</span>
								</div>
							</div>
						</div>

						<!-- Footer -->
						<div class="border-t border-gray-100 p-4 dark:border-gray-800">
							<div class="flex items-center justify-between">
								<div class="flex items-center gap-2">
									{#if data.profile.profile_image}
										<img
											src={data.profile.profile_image}
											alt={data.profile.name}
											class="h-8 w-8 rounded-full object-cover"
										/>
									{:else}
										<div class="flex h-8 w-8 items-center justify-center rounded-full bg-gray-100 dark:bg-gray-800">
											<span class="text-xs font-medium text-gray-600 dark:text-gray-400">
												{data.profile.name.charAt(0).toUpperCase()}
											</span>
										</div>
									{/if}
									<span class="text-sm font-medium text-gray-900 dark:text-white">{data.profile.name}</span>
								</div>
								<div class="flex items-center gap-1 text-red-500">
									<svg class="h-4 w-4" fill="currentColor" viewBox="0 0 24 24">
										<path
											d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z"
										/>
									</svg>
									<span class="text-sm font-medium">19</span>
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>

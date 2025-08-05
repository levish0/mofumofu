<script lang="ts">
	import { goto } from '$app/navigation';
	import { authStore } from '$lib/stores/auth.svelte.js';

	type Props = {
		profile: {
			name: string;
			handle: string;
			banner_image?: string;
			profile_image?: string;
		};
		isOwnProfile: boolean;
		isLoading: boolean;
		topPosition: string;
	};

	const { profile, isOwnProfile, isLoading, topPosition }: Props = $props();

	let bannerLoaded = $state(false);
	let profileImageLoaded = $state(false);

	function handleEditProfile() {
		goto('/settings');
	}

	function handleFollow() {
		console.log('Follow user:', profile.handle);
	}
</script>

<div class="transition-all duration-100 ease-out">
	<!-- Banner Section -->
	<div class="relative aspect-[3/1] w-full">
		{#if profile.banner_image}
			{#if !bannerLoaded}
				<div class="shimmer bg-mofu-dark-900 absolute inset-0 overflow-hidden rounded-xl"></div>
			{/if}
			<img
				src={profile.banner_image}
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
				{#if profile.profile_image}
					{#if !profileImageLoaded}
						<div
							class="shimmer bg-mofu-dark-900 border-mofu-dark-900 absolute inset-0 rounded-full border-4"
						></div>
					{/if}
					<img
						src={profile.profile_image}
						alt={profile.name}
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
							{profile.name.charAt(0).toUpperCase()}
						</span>
					</div>
				{/if}
			</div>
		</div>
	</div>
</div>
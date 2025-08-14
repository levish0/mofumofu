<script lang="ts">
	import { goto } from '$app/navigation';
	import { authStore } from '$lib/stores/auth.svelte.js';
	import { Button } from '../ui/button';
	import type { UserInfoResponse } from '$lib/api/user/types';
	import { checkFollowStatus, createFollow, deleteFollow } from '$lib/api';
	import * as m from '../../../paraglide/messages';

	type Props = {
		profile: UserInfoResponse;
		isOwnProfile: boolean;
		isLoading: boolean;
		topPosition: string;
	};

	const { profile, isOwnProfile, isLoading, topPosition }: Props = $props();

	let bannerLoaded = $state(false);
	let bannerError = $state(false);
	let bannerRetryCount = $state(0);
	let bannerRetryTimer: ReturnType<typeof setTimeout> | null = null;
	
	let profileImageLoaded = $state(false);
	let profileImageError = $state(false);
	let profileImageRetryCount = $state(0);
	let profileImageRetryTimer: ReturnType<typeof setTimeout> | null = null;
	
	let isFollowing = $state(false);
	let followLoading = $state(false);
	let followStatusLoaded = $state(false);

	function handleEditProfile() {
		goto('/settings');
	}

	async function loadFollowStatus() {
		if (!authStore.isAuthenticated || isOwnProfile) {
			return;
		}

		try {
			const status = await checkFollowStatus({ handle: profile.handle });
			isFollowing = status.is_following;
		} catch (error) {
			console.error('Failed to check follow status:', error);
		} finally {
			followStatusLoaded = true;
		}
	}

	async function handleFollowToggle() {
		if (!authStore.isAuthenticated) {
			goto('/account/signin');
			return;
		}

		followLoading = true;
		try {
			if (isFollowing) {
				await deleteFollow({ followee_handle: profile.handle });
				isFollowing = false;
			} else {
				await createFollow({ followee_handle: profile.handle });
				isFollowing = true;
			}
		} catch (error) {
			console.error('Failed to toggle follow:', error);
		} finally {
			followLoading = false;
		}
	}

	function handleBannerError() {
		if (bannerRetryCount < 3) {
			// Retry after 2, 4, 8 seconds
			const retryDelay = Math.pow(2, bannerRetryCount + 1) * 1000;
			bannerRetryTimer = setTimeout(() => {
				bannerRetryCount++;
				bannerError = false; // Force re-render
				bannerLoaded = false;
			}, retryDelay);
		} else {
			bannerError = true; // Give up and show fallback
			bannerLoaded = true;
		}
	}

	function handleProfileImageError() {
		if (profileImageRetryCount < 3) {
			// Retry after 2, 4, 8 seconds
			const retryDelay = Math.pow(2, profileImageRetryCount + 1) * 1000;
			profileImageRetryTimer = setTimeout(() => {
				profileImageRetryCount++;
				profileImageError = false; // Force re-render
				profileImageLoaded = false;
			}, retryDelay);
		} else {
			profileImageError = true; // Give up and show fallback
			profileImageLoaded = true;
		}
	}

	// Load follow status when component mounts or authentication state changes
	$effect(() => {
		loadFollowStatus();
	});

	// Reset error states and retry counts when image URLs change
	$effect(() => {
		if (profile.banner_image) {
			bannerError = false;
			bannerLoaded = false;
			bannerRetryCount = 0;
			if (bannerRetryTimer) {
				clearTimeout(bannerRetryTimer);
				bannerRetryTimer = null;
			}
		}
	});

	$effect(() => {
		if (profile.profile_image) {
			profileImageError = false;
			profileImageLoaded = false;
			profileImageRetryCount = 0;
			if (profileImageRetryTimer) {
				clearTimeout(profileImageRetryTimer);
				profileImageRetryTimer = null;
			}
		}
	});
</script>

<div class="transition-all duration-100 ease-out">
	<!-- Banner Section -->
	<div class="relative aspect-[3/1] w-full">
		{#if profile.banner_image && !bannerError}
			{#if !bannerLoaded}
				<div class="shimmer dark:bg-mofu-dark-900 bg-mofu-light-100 absolute inset-0 overflow-hidden rounded-xl"></div>
			{/if}
			<img
				src={profile.banner_image}
				alt="Banner"
				class="absolute inset-0 h-full w-full overflow-hidden rounded-xl object-cover transition-opacity duration-300"
				class:opacity-0={!bannerLoaded}
				class:opacity-100={bannerLoaded}
				loading="lazy"
				onload={() => (bannerLoaded = true)}
				onerror={handleBannerError}
			/>
		{:else}
			<div class="h-full w-full overflow-hidden rounded-xl bg-gradient-to-r from-blue-400 to-purple-500"></div>
		{/if}

		<!-- Action Button (next to profile image) -->
		<div class="absolute right-4 -bottom-12 z-20">
			{#if isLoading}
				<div class="shimmer h-10 w-20 rounded-full"></div>
			{:else if isOwnProfile}
				<Button variant="outline" onclick={handleEditProfile} class=" bg-transparent px-3 py-0">Edit Profile</Button>
			{:else if !followStatusLoaded}
				<div class="shimmer h-10 w-20 rounded-full"></div>
			{:else}
				<Button
					onclick={handleFollowToggle}
					disabled={followLoading}
					variant={isFollowing ? 'outline' : 'default'}
					class="px-3 py-0 {isFollowing ? 'bg-transparent' : 'dark:text-mofu-dark-900'}"
				>
					{#if followLoading}
						...
					{:else if !authStore.isAuthenticated}
						{m.profile_sign_in_to_follow()}
					{:else if isFollowing}
						Unfollow
					{:else}
						{m.profile_follow()}
					{/if}
				</Button>
			{/if}
		</div>

		<!-- Profile Image (overlapping banner) -->
		<div class="absolute -bottom-12 left-4 z-10">
			<div class="relative h-24 w-24">
				{#if profile.profile_image && !profileImageError}
					{#if !profileImageLoaded}
						<div
							class="shimmer dark:border-mofu-dark-900 dark:bg-mofu-dark-900 bg-mofu-light-100 border-mofu-light-100 absolute inset-0 rounded-full border-4"
						></div>
					{/if}
					<img
						src={profile.profile_image}
						alt={profile.name}
						class="dark:border-mofu-dark-900 dark:bg-mofu-dark-900 bg-mofu-light-100 border-mofu-light-100 absolute inset-0 h-24 w-24 rounded-full border-4 object-cover transition-opacity duration-300"
						class:opacity-0={!profileImageLoaded}
						class:opacity-100={profileImageLoaded}
						loading="lazy"
						onload={() => (profileImageLoaded = true)}
						onerror={handleProfileImageError}
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

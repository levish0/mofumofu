<script lang="ts">
	import { getContext } from 'svelte';
	import { authStore } from '$lib/stores/auth.svelte.js';
	import { userStore } from '$lib/stores/user.svelte';
	import ProfileHeader from '$lib/components/profile/ProfileHeader.svelte';
	import ProfileInfo from '$lib/components/profile/ProfileInfo.svelte';
	import ProfilePostGrid from '$lib/components/profile/ProfilePostGrid.svelte';
	import type { PageData } from './$types';

	const { data }: { data: PageData } = $props();

	let isLoading = $state(true);
	let isOwnProfile = $state(false);

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
					// Ensure user profile is loaded
					if (!userStore.user) {
						await userStore.loadProfile();
					}

					if (userStore.user) {
						isOwnProfile = userStore.user.handle === data.profile.handle;
					} else {
						isOwnProfile = false;
					}
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
</script>

<svelte:head>
	<title>{data.profile.name} (@{data.profile.handle}) - Mofu</title>
	<meta name="description" content={data.profile.bio || `${data.profile.name}의 프로필을 확인하세요.`} />

	<!-- Open Graph -->
	<meta property="og:title" content="{data.profile.name} (@{data.profile.handle}) - Mofu" />
	<meta property="og:description" content={data.profile.bio || `${data.profile.name}의 프로필을 확인하세요.`} />
	<meta property="og:type" content="profile" />
	<meta property="og:url" content="https://mofumofu.ink/{data.profile.handle}/profile" />
	{#if data.profile.profile_image}
		<meta property="og:image" content={data.profile.profile_image} />
	{:else}
		<meta property="og:image" content="https://mofumofu.ink/og-default.png" />
	{/if}
	<meta property="og:site_name" content="Mofu" />
	<meta property="profile:username" content={data.profile.handle} />

	<!-- Twitter Card -->
	<meta name="twitter:card" content="summary" />
	<meta name="twitter:title" content="{data.profile.name} (@{data.profile.handle}) - Mofu" />
	<meta name="twitter:description" content={data.profile.bio || `${data.profile.name}의 프로필을 확인하세요.`} />
	{#if data.profile.profile_image}
		<meta name="twitter:image" content={data.profile.profile_image} />
	{:else}
		<meta name="twitter:image" content="https://mofumofu.ink/og-default.png" />
	{/if}
</svelte:head>
<div class="min-h-screen">
	<div class="max-w-8xl mx-auto px-4 pt-2">
		<!-- Two Column Layout -->
		<div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
			<!-- Left Column: Profile Info -->
			<div class="lg:col-span-1">
				<div class="sticky transition-all duration-100 ease-out" style="top: {topPosition}">
					<ProfileHeader profile={data.profile} {isOwnProfile} {isLoading} {topPosition} />
					<ProfileInfo profile={data.profile} followerCount={data.followerCount} followingCount={data.followingCount} />
				</div>
			</div>

			<!-- Right Column: Posts -->
			<div class="lg:col-span-1">
				<ProfilePostGrid profile={data.profile} posts={data.posts} />
			</div>
		</div>
	</div>
</div>

<script lang="ts">
	import { getContext } from 'svelte';
	import { getMyProfile } from '$lib/api/user/userApi';
	import { authStore } from '$lib/stores/auth.svelte.js';
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
</script>

<svelte:head>
	<title>{data.profile.name} (@{data.profile.handle}) - Mofu</title>
</svelte:head>
<div class="min-h-screen">
	<div class="max-w-8xl mx-auto px-4 pt-2">
		<!-- Two Column Layout -->
		<div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
			<!-- Left Column: Profile Info -->
			<div class="lg:col-span-1">
				<div class="sticky transition-all duration-100 ease-out" style="top: {topPosition}">
					<ProfileHeader 
						profile={data.profile} 
						{isOwnProfile} 
						{isLoading} 
						{topPosition} 
					/>
					<ProfileInfo profile={data.profile} />
				</div>
			</div>

			<!-- Right Column: Posts -->
			<div class="lg:col-span-1">
				<ProfilePostGrid profile={data.profile} />
			</div>
		</div>
	</div>
</div>

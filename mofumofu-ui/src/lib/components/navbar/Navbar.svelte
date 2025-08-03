<!-- src/lib/components/Navbar.svelte -->
<script lang="ts">
	import { Cog6Tooth, Icon } from 'svelte-hero-icons';
	import {
		ArrowTrendingUp,
		Clock,
		Rss,
		Bell,
		MagnifyingGlass,
		ChevronDown,
		User,
		ArrowRightOnRectangle
	} from 'svelte-hero-icons';
	import { getMyProfile } from '$lib/api/user/userApi';
	import { onMount } from 'svelte';
	import { authStore } from '$lib/stores/auth.svelte';
	import type { UserInfoResponse } from '$lib/api/user/types';
	import { signOut } from '$lib/api/auth/authApi';
	import { fly, scale } from 'svelte/transition';
	import { Button } from '../ui/button';
	import { goto, invalidateAll } from '$app/navigation';

	let { isVisible, isAtTop } = $props();

	let userInfo: UserInfoResponse | null = $state(null);
	let isLoading = $state(false);
	let isDropdownOpen = $state(false);
	let closeTimer: number | null = null;

	async function loadUserProfile() {
		if (isLoading || userInfo) return;

		isLoading = true;

		try {
			console.log('loading user profile from navbar');
			userInfo = await getMyProfile();
			// API 호출이 성공하면 토큰이 있다는 뜻이므로 authStore 업데이트
			// (실제 토큰은 API 호출 과정에서 이미 설정됨)
		} catch (error) {
			console.error('Failed to fetch user profile:', error);
			userInfo = null;
		} finally {
			isLoading = false;
		}
	}

	onMount(() => {
		loadUserProfile();
	});

	async function handleLogout() {
		try {
			await signOut();
			authStore.clearToken();
			userInfo = null;
			isDropdownOpen = false;
			await invalidateAll();
			goto('/');
		} catch (error) {
			console.error('Logout failed:', error);
			return;
		}
	}

	function openDropdown() {
		if (closeTimer) {
			clearTimeout(closeTimer);
			closeTimer = null;
		}
		isDropdownOpen = true;
	}

	function scheduleClose() {
		closeTimer = setTimeout(() => {
			isDropdownOpen = false;
			closeTimer = null;
		}, 100);
	}
</script>

<nav
	class="fixed top-0 right-0 left-0 z-50 max-h-[60px] w-full transition-all duration-100 ease-out"
	class:bg-mofu-dark-800={!isAtTop()}
	class:bg-mofu-dark-900={isAtTop()}
	style="transform: translateY({isVisible() ? '0' : '-100%'});"
>
	<div class="max-w-8xl mx-auto flex items-center justify-between px-4 py-3">
		<!-- 좌측 -->
		<div class="flex items-center space-x-3">
			<a href="/" class="text-3xl font-bold whitespace-nowrap text-white">もふもふ。</a>
		</div>

		<!-- 우측 -->
		<div class="flex items-center space-x-3">
			{#if isLoading}
				<div class="shimmer h-9 w-9 rounded-full"></div>
				<div class="shimmer hidden h-9 w-9 rounded-full sm:block"></div>
				<div class="shimmer h-9 w-18 rounded-full sm:hidden"></div>
				<div class="shimmer hidden h-9 w-28 rounded-full sm:block"></div>
				<div class="shimmer hidden h-9 w-9 rounded-full sm:block"></div>
				<div class="shimmer hidden h-9 w-9 rounded-full sm:block"></div>
			{:else if userInfo}
				<Button href="/" variant="icon" aria-label="notifications">
					<Icon src={Bell} size="20" class="text-white" />
				</Button>

				<Button href="/" variant="icon" aria-label="search">
					<Icon src={MagnifyingGlass} size="20" class="text-white" />
				</Button>

				<Button href="/write" variant="outline" class="bg-transparent px-3 py-0">새 글 작성하기</Button>

				<div
					class=" dropdown-container relative"
					role="button"
					tabindex="0"
					onmouseenter={openDropdown}
					onmouseleave={scheduleClose}
				>
					<button class="flex h-9 items-center space-x-1 rounded-lg" aria-label="profile_menu">
						<div class="h-9 w-9 overflow-hidden rounded-full">
							{#if userInfo.profile_image}
								<img src={userInfo.profile_image} alt="{userInfo.handle}의 프로필" class="h-full w-full object-cover" />
							{:else}
								<span class="flex h-full w-full items-center justify-center text-sm font-medium text-white">
									{userInfo.handle.charAt(0).toUpperCase()}
								</span>
							{/if}
						</div>
					</button>

					{#if isDropdownOpen}
						<div
							class="bg-mofu-dark-800 absolute top-14 right-0 z-50 w-48 rounded-lg text-sm font-bold shadow-lg"
							transition:fly={{ y: -8, duration: 150 }}
							style="transform-origin: top right;"
						>
							<div class="py-1">
								<a
									href="/profile/{userInfo.handle}"
									class="dark:text-mofu-dark-200 hover:text-mofu flex items-center px-4 py-2"
								>
									<Icon src={User} solid size="16" class="mr-3" />
									마이페이지
								</a>
								<a href="/settings" class="dark:text-mofu-dark-200 hover:text-mofu flex items-center px-4 py-2">
									<Icon src={Cog6Tooth} solid size="16" class="mr-3" />
									설정
								</a>
								<button
									class="dark:text-mofu-dark-200 hover:text-mofu flex w-full items-center px-4 py-2"
									onclick={handleLogout}
								>
									<Icon src={ArrowRightOnRectangle} solid size="16" class="mr-3" />
									로그아웃
								</button>
							</div>
						</div>
					{/if}
				</div>
			{:else}
				<Button href="/" variant="icon" aria-label="search">
					<Icon src={MagnifyingGlass} size="20" class="text-white" />
				</Button>
				<Button href="/settings" variant="icon" aria-label="settings">
					<Icon src={Cog6Tooth} solid size="20" class="text-white" />
				</Button>
				<Button href="/account/signup" class="py-0">로그인</Button>
			{/if}
		</div>
	</div>
</nav>

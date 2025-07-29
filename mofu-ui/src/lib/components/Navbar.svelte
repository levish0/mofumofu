<!-- src/lib/components/Navbar.svelte -->
<script lang="ts">
	import { Icon } from 'svelte-hero-icons';
	import {
		ArrowTrendingUp,
		Clock,
		Rss,
		Bookmark,
		Bell,
		MagnifyingGlass,
		ChevronDown,
		Pencil,
		EllipsisVertical
	} from 'svelte-hero-icons';
	import { getMyProfile } from '$lib/api/user/userApi';
	import { onMount } from 'svelte';
	import { authStore } from '$lib/stores/auth.svelte';

	interface UserInfoResponse {
		handle: string;
		profile_image?: string;
	}

	let userInfo: UserInfoResponse | null = $state(null);
	let isLoading = $state(false);
	let errorLoadingUser = $state(false);

	async function loadUserProfile() {
		if (isLoading || userInfo) return;

		isLoading = true;
		errorLoadingUser = false;

		try {
			userInfo = await getMyProfile();
			console.log('User profile loaded:', userInfo);
		} catch (error) {
			console.error('Failed to fetch user profile:', error);
			errorLoadingUser = true;
			userInfo = null;
		} finally {
			isLoading = false;
		}
	}

	onMount(async () => {
		console.log('Navbar mounted, attempting to load user profile');
		await loadUserProfile();
	});

	$effect(() => {
		if (!authStore.isAuthenticated && userInfo) {
			userInfo = null;
			errorLoadingUser = false;
		}
	});
</script>

<nav class="bg-mofu-dark-800 w-full text-white">
	<div class="mx-auto flex max-w-7xl items-center justify-between px-4 py-3">
		<!-- 좌측 -->
		<div class="flex items-center space-x-3">
			<div class="text-3xl font-bold">もふもふ。</div>
		</div>

		<!-- 우측 -->
		<div class="flex items-center space-x-3">
			{#if isLoading}
				<!-- 스켈레톤: 고정 높이 36px -->
				<div class="h-9 w-9 animate-pulse rounded-full bg-white/10"></div>
				<div class="h-9 w-9 animate-pulse rounded-full bg-white/10"></div>
				<div class="h-9 w-28 animate-pulse rounded-full bg-white/10"></div>
				<div class="h-9 w-9 animate-pulse rounded-full bg-white/10"></div>
				<div class="h-9 w-9 animate-pulse rounded bg-white/10"></div>
			{:else if userInfo}
				<button class="h-9 w-9 rounded-full p-2 transition-colors hover:bg-white/10" aria-label="알림">
					<Icon src={Bell} size="20" class="text-white" />
				</button>

				<button class="h-9 w-9 rounded-full p-2 transition-colors hover:bg-white/10" aria-label="검색">
					<Icon src={MagnifyingGlass} size="20" class="text-white" />
				</button>

				<button
					class="h-9 rounded-full border-2 border-white px-3 font-bold transition-colors hover:bg-white hover:text-black"
					aria-label="새 글 작성하기"
				>
					새 글 작성하기
				</button>

				<button
					class="flex h-9 items-center space-x-1 rounded-lg px-2 transition-colors hover:bg-white/10"
					aria-label="프로필 메뉴"
				>
					<div class="h-9 w-9 overflow-hidden rounded-full bg-red-500">
						{#if userInfo.profile_image}
							<img src={userInfo.profile_image} alt="{userInfo.handle}의 프로필" class="h-full w-full object-cover" />
						{:else}
							<span class="flex h-full w-full items-center justify-center text-sm font-medium text-white">
								{userInfo.handle.charAt(0).toUpperCase()}
							</span>
						{/if}
					</div>
					<Icon src={ChevronDown} size="16" class="text-gray-400" />
				</button>
			{:else}
				<button class="h-9 w-9 rounded-full p-2 transition-colors hover:bg-white/10" aria-label="검색">
					<Icon src={MagnifyingGlass} size="20" class="text-white" />
				</button>

				<a
					href="/account/signin"
					class="inline-flex h-9 items-center justify-center rounded-full border-2 border-white bg-white px-4 font-bold text-black transition-colors hover:opacity-70"
					aria-label="로그인"
				>
					로그인
				</a>
			{/if}
		</div>
	</div>

	<!-- 모바일 하단 네비 -->
	<div class="border-t border-gray-700 px-4 py-2 md:hidden">
		<div class="flex space-x-4">
			<a
				href="/"
				class="flex items-center space-x-1 text-sm transition-colors hover:text-orange-400"
				aria-label="트렌딩 페이지로 이동"
			>
				<Icon src={ArrowTrendingUp} size="16" />
				<span>트렌딩</span>
			</a>

			<a
				href="/recent"
				class="flex items-center space-x-1 text-sm transition-colors hover:text-orange-400"
				aria-label="최신 페이지로 이동"
			>
				<Icon src={Clock} size="16" />
				<span>최신</span>
			</a>

			<a
				href="/feed"
				class="flex items-center space-x-1 text-sm transition-colors hover:text-orange-400"
				aria-label="피드 페이지로 이동"
			>
				<Icon src={Rss} size="16" />
				<span>피드</span>
			</a>
		</div>
	</div>
</nav>

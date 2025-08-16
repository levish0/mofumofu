<script lang="ts">
	import {
		User,
		ComputerDesktop,
		Bell,
		ShieldExclamation,
		CheckCircle,
		ArrowUturnLeft,
		Trash,
		ArrowDownTray,
		ArrowUpTray,
		Icon,
		CreditCard,
		PencilSquare
	} from 'svelte-hero-icons';
	import { getContext, onMount } from 'svelte';
	import { settingsStore } from '$lib/stores/settings.svelte';
	import { authStore } from '$lib/stores/auth.svelte';
	import { userStore } from '$lib/stores/user.svelte';
	import MobileSettingsLayout from '$lib/components/settings/layouts/MobileSettingsLayout.svelte';
	import DesktopSettingsLayout from '$lib/components/settings/layouts/DesktopSettingsLayout.svelte';
	import * as m from '../../../paraglide/messages';

	let selectedSection = $state(authStore.isAuthenticated ? 'personal' : 'display');
	let saveSuccess = $state(false);

	// 모바일에서 accordion의 기본 열린 섹션
	let accordionValue = $state(authStore.isAuthenticated ? 'personal' : 'display');

	type NavbarContext = {
		isVisible: () => boolean;
		isAtTop: () => boolean;
	};

	const navbar = getContext<NavbarContext>('navbar');

	// Calculate the top position based on navbar state
	const topPosition = $derived(navbar.isVisible() ? '68px' : '8px');

	const sections = [
		{
			id: 'personal',
			label: () => m.settings_personal_info(),
			icon: User,
			description: () => m.settings_personal_info_desc(),
			requiresAuth: true
		},
		{
			id: 'account',
			label: () => m.settings_account(),
			icon: CreditCard,
			description: () => m.settings_account_desc(),
			requiresAuth: true
		},
		{
			id: 'display',
			label: () => m.settings_display(),
			icon: ComputerDesktop,
			description: () => m.settings_display_desc(),
			requiresAuth: false
		},
		{
			id: 'writing',
			label: () => m.settings_writing(),
			icon: PencilSquare,
			description: () => m.settings_writing_desc(),
			requiresAuth: true
		},
		{
			id: 'notifications',
			label: () => m.settings_notifications(),
			icon: Bell,
			description: () => m.settings_notifications_desc(),
			requiresAuth: true
		},
		{
			id: 'privacy',
			label: () => m.settings_privacy(),
			icon: ShieldExclamation,
			description: () => m.settings_privacy_desc(),
			requiresAuth: true
		}
	];

	// URL 해시에서 섹션 읽기, 없으면 기본값 사용
	const getInitialSection = () => {
		if (typeof window !== 'undefined') {
			const hash = window.location.hash.slice(1); // # 제거
			if (hash && sections.some((s) => s.id === hash)) {
				return hash;
			}
		}
		return authStore.isAuthenticated ? 'personal' : 'display';
	};

	onMount(() => {
		// URL 해시에서 초기 섹션 설정
		selectedSection = getInitialSection();

		// Initialize settings with default data
		settingsStore.initializeWithDefaults();
	});

	// userStore.user가 로드되면 자동으로 settings 업데이트
	let userInitialized = $state(false);

	$effect(() => {
		if (authStore.isAuthenticated && userStore.user && !userInitialized) {
			settingsStore.updatePersonalSilent({
				handle: userStore.user.handle,
				name: userStore.user.name,
				bio: userStore.user.bio || '',
				location: userStore.user.location || '',
				website: userStore.user.website || '',
				profileImage: userStore.user.profile_image || null,
				bannerImage: userStore.user.banner_image || null
			});

			// API 호출이 성공하면 인증된 상태이므로 personal 섹션으로 변경 (단, 해시가 없는 경우만)
			if (!window.location.hash && selectedSection === 'display') {
				selectedSection = 'personal';
				window.location.hash = 'personal';
			}

			userInitialized = true;
		}
	});

	async function handleSave() {
		const result = await settingsStore.saveChanges();
		if (result.success) {
			saveSuccess = true;
		} else {
			saveSuccess = false;
		}
	}

	// Reset save success when user makes new changes
	$effect(() => {
		if (settingsStore.hasChanges && saveSuccess) {
			saveSuccess = false;
		}
	});

	function handleSectionChange(sectionId: string) {
		selectedSection = sectionId;
	}
</script>

<svelte:head>
	<title>설정 - Mofumofu</title>
	<meta name="description" content="Mofumofu의 개인정보, 계정, 디스플레이, 알림, 개인정보 보호 설정을 관리하세요." />
	<meta name="robots" content="noindex, nofollow" />

	<!-- Open Graph -->
	<meta property="og:title" content="설정 - Mofumofu" />
	<meta
		property="og:description"
		content="Mofumofu의 개인정보, 계정, 디스플레이, 알림, 개인정보 보호 설정을 관리하세요."
	/>
	<meta property="og:type" content="website" />
	<meta property="og:site_name" content="Mofumofu" />

	<!-- Twitter Card -->
	<meta name="twitter:card" content="summary" />
	<meta name="twitter:title" content="설정 - Mofumofu" />
	<meta
		name="twitter:description"
		content="Mofumofu의 개인정보, 계정, 디스플레이, 알림, 개인정보 보호 설정을 관리하세요."
	/>
</svelte:head>

<!-- 데스크톱 레이아웃 -->
<DesktopSettingsLayout 
	{sections} 
	{selectedSection} 
	{topPosition} 
	{handleSave} 
	{saveSuccess}
	onSectionChange={handleSectionChange} 
/>

<!-- 모바일 레이아웃 -->
<MobileSettingsLayout 
	{sections} 
	{handleSave} 
	{saveSuccess}
/>

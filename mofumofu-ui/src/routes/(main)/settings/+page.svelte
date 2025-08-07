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
	import { getMyProfile } from '$lib/api/user/userApi';
	import { authStore } from '$lib/stores/auth.svelte';
	import { PersonalInfoSettings } from '$lib/components/settings/PersonalInfoSettings';
	import AccountSettings from '$lib/components/settings/AccountSettings.svelte';
	import DisplaySettings from '$lib/components/settings/DisplaySettings.svelte';
	import WritingSettings from '$lib/components/settings/WritingSettings.svelte';
	import PrivacySettings from '$lib/components/settings/PrivacySettings.svelte';
	import NotificationSettings from '$lib/components/settings/NotificationSettings.svelte';
	import * as m from '../../../paraglide/messages';

	let selectedSection = $state(authStore.isAuthenticated ? 'personal' : 'display');
	let saveSuccess = $state(false);

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

	onMount(async () => {
		// URL 해시에서 초기 섹션 설정
		selectedSection = getInitialSection();

		// Initialize settings with default data
		settingsStore.initializeWithDefaults();

		// Load user profile data from API
		try {
			const userProfile = await getMyProfile();
			settingsStore.updatePersonalSilent({
				handle: userProfile.handle,
				name: userProfile.name,
				bio: userProfile.bio || '',
				location: userProfile.location || '',
				website: userProfile.website || '',
				profileImage: userProfile.profile_image || null,
				bannerImage: userProfile.banner_image || null
			});
			// API 호출이 성공하면 인증된 상태이므로 personal 섹션으로 변경 (단, 해시가 없는 경우만)
			if (!window.location.hash && selectedSection === 'display') {
				selectedSection = 'personal';
				window.location.hash = 'personal';
			}
		} catch (error) {
			console.error('Failed to load user profile:', error);
			// Keep default data if API call fails
		}
	});

	async function handleSave() {
		const result = await settingsStore.saveChanges();
		if (result.success) {
			saveSuccess = true;
			// Show success message briefly then reload
			setTimeout(() => {
				window.location.reload();
			}, 1000);
		}
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

<div class="text-mofu-dark-200 flex min-h-screen w-full gap-4">
	<!-- Sidebar with Card Grid -->
	<div class="w-1/3">
		<div class="sticky space-y-4 transition-all duration-100 ease-out" style="top: {topPosition}">
			<div class="grid grid-cols-1 gap-3 sm:grid-cols-2">
				{#each sections as section}
					<button
						class="dark:bg-mofu-dark-800 dark:border-mofu-dark-800 group flex cursor-pointer flex-col overflow-hidden rounded-xl border p-4 text-left transition-all duration-200 hover:opacity-75 hover:shadow-xl {selectedSection ===
						section.id
							? 'opacity-70'
							: ''} {section.requiresAuth && !authStore.isAuthenticated ? 'cursor-not-allowed opacity-30' : ''}"
						onclick={() => {
							if (!section.requiresAuth || authStore.isAuthenticated) {
								selectedSection = section.id;
								window.location.hash = section.id;
							}
						}}
					>
						<div class="mb-3 flex items-center gap-3">
							<div class="rounded-lg bg-white/10 p-2">
								<Icon src={section.icon} size="20" solid class="text-mofu" />
							</div>
							<div class="flex-1">
								<h3 class="text-mofu-dark-200 text-md font-bold break-all">{section.label()}</h3>
							</div>
						</div>
						<p class="text-xs text-gray-400">{section.description()}</p>
					</button>
				{/each}
			</div>

			<!-- Save Button -->
			<button
				class="dark:bg-mofu-dark-800 dark:border-mofu-dark-800 group flex w-full cursor-pointer flex-col overflow-hidden rounded-xl border p-4 text-left transition-all duration-200 hover:opacity-75 hover:shadow-xl {!settingsStore.hasChanges
					? 'cursor-not-allowed opacity-50'
					: ''}"
				onclick={handleSave}
				disabled={!settingsStore.hasChanges || settingsStore.isLoading}
			>
				<div class="flex items-center justify-center gap-2">
					{#if settingsStore.isLoading}
						<svg class="h-4 w-4 animate-spin" fill="none" viewBox="0 0 24 24">
							<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
							<path
								class="opacity-75"
								fill="currentColor"
								d="m4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
							></path>
						</svg>
						<h3 class="text-mofu-dark-200 text-md font-bold">{m.settings_saving()}</h3>
					{:else if saveSuccess}
						<Icon src={CheckCircle} class="h-4 w-4 text-green-400" />
						<h3 class="text-mofu-dark-200 text-md font-bold">{m.settings_saved()}</h3>
					{:else}
						<h3 class="text-mofu-dark-200 text-md font-bold">{m.settings_save_changes()}</h3>
						{#if settingsStore.hasChanges}
							<span class="text-xs text-orange-400">•</span>
						{/if}
					{/if}
				</div>
			</button>

			<!-- Reset Button (only show if there are changes) -->
			{#if settingsStore.hasChanges}
				<button
					class="dark:bg-mofu-dark-800/50 dark:border-mofu-dark-800 group flex w-full cursor-pointer flex-col overflow-hidden rounded-xl border p-2 text-center transition-all duration-200 hover:opacity-75"
					onclick={() => settingsStore.resetChanges()}
				>
					<div class="flex items-center justify-center gap-2">
						<Icon src={ArrowUturnLeft} class="h-4 w-4" />
						<span class="text-sm text-gray-400">{m.settings_reset_changes()}</span>
					</div>
				</button>
			{/if}

			<!-- Error Messages -->
			{#if settingsStore.errors.general}
				<div class="rounded-lg border border-red-500/20 bg-red-500/10 p-3">
					<p class="text-xs text-red-400">{settingsStore.errors.general}</p>
				</div>
			{/if}

			<!-- Validation Errors -->
			{#if settingsStore.hasValidationErrors()}
				<div class="rounded-lg border border-orange-500/20 bg-orange-500/10 p-3">
					<p class="text-xs font-medium text-orange-400">{m.settings_validation_errors()}</p>
					{#each Object.entries(settingsStore.validationErrors) as [section, sectionErrors]}
						{#if Object.keys(sectionErrors).length > 0}
							<div class="mt-2">
								{#each Object.entries(sectionErrors) as [field, error]}
									<p class="text-xs text-orange-400">• {field}: {error}</p>
								{/each}
							</div>
						{/if}
					{/each}
				</div>
			{/if}
		</div>
	</div>

	<!-- Main Content -->
	<div class="flex-1">
		{#if selectedSection === 'personal'}
			<PersonalInfoSettings />
		{:else if selectedSection === 'account'}
			<AccountSettings />
		{:else if selectedSection === 'display'}
			<DisplaySettings />
		{:else if selectedSection === 'writing'}
			<WritingSettings />
		{:else if selectedSection === 'notifications'}
			<NotificationSettings />
		{:else if selectedSection === 'privacy'}
			<PrivacySettings />
		{/if}
	</div>
</div>

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
	import { PersonalInfoSettings } from '$lib/components/settings/PersonalInfoSettings';
	import AccountSettings from '$lib/components/settings/AccountSettings.svelte';
	import DisplaySettings from '$lib/components/settings/DisplaySettings.svelte';
	import WritingSettings from '$lib/components/settings/WritingSettings.svelte';
	import PrivacySettings from '$lib/components/settings/PrivacySettings.svelte';
	import NotificationSettings from '$lib/components/settings/NotificationSettings.svelte';
	import * as Accordion from '$lib/components/ui/accordion';
	import { Button } from '$lib/components/ui/button';
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

<!-- 데스크톱 레이아웃 (기존) -->
<div class="text-mofu-dark-200 hidden min-h-screen w-full gap-4 lg:flex">
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
						<p class="dark:text-mofu-dark-300 text-xs">{section.description()}</p>
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
						<span class="dark:text-mofu-dark-300 text-sm">{m.settings_reset_changes()}</span>
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

<!-- 모바일 레이아웃 (Accordion) -->
<div class="text-mofu-dark-200 min-h-screen w-full pb-10 lg:hidden">
	<Accordion.Root type="single" bind:value={accordionValue} class="space-y-2">
		{#each sections.filter((s) => !s.requiresAuth || authStore.isAuthenticated) as section}
			<Accordion.Item value={section.id}>
				<Accordion.Trigger class="flex w-full items-center justify-between  text-left">
					<div class="flex items-center gap-3">
						<div class="rounded-lg bg-white/10 p-2">
							<Icon src={section.icon} size="20" solid class="text-mofu" />
						</div>
						<div>
							<h3 class="text-mofu-dark-200 text-md font-bold">{section.label()}</h3>
							<p class="dark:text-mofu-dark-300 text-xs">{section.description()}</p>
						</div>
					</div>
				</Accordion.Trigger>
				<Accordion.Content class=" pb-4">
					{#if section.id === 'personal'}
						<PersonalInfoSettings />
					{:else if section.id === 'account'}
						<AccountSettings />
					{:else if section.id === 'display'}
						<DisplaySettings />
					{:else if section.id === 'writing'}
						<WritingSettings />
					{:else if section.id === 'notifications'}
						<NotificationSettings />
					{:else if section.id === 'privacy'}
						<PrivacySettings />
					{/if}
				</Accordion.Content>
			</Accordion.Item>
		{/each}
	</Accordion.Root>

	<!-- Error Messages -->
	{#if settingsStore.errors.general}
		<div class="mt-4 rounded-lg border border-red-500/20 bg-red-500/10 p-3">
			<p class="text-xs text-red-400">{settingsStore.errors.general}</p>
		</div>
	{/if}

	<!-- Validation Errors -->
	{#if settingsStore.hasValidationErrors()}
		<div class="mt-4 rounded-lg border border-orange-500/20 bg-orange-500/10 p-3">
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

<!-- 모바일용 하단 고정 Save Changes 버튼 -->
<div class="dark:text-mofu-dark-200 fixed right-4 bottom-4 left-4 z-50 space-y-2 lg:hidden">
	<!-- Save Button -->
	<Button
		class="dark:bg-mofu-dark-800  group flex w-full cursor-pointer flex-col overflow-hidden rounded-xl border px-6 py-6 text-left transition-all duration-200 hover:opacity-75 hover:shadow-xl {!settingsStore.hasChanges
			? 'cursor-not-allowed '
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
	</Button>

	<!-- Reset Button (only show if there are changes) -->
	{#if settingsStore.hasChanges}
		<Button
			class="dark:bg-mofu-dark-800 group flex w-full cursor-pointer flex-col overflow-hidden rounded-xl border px-6 py-6 text-center transition-all duration-200 hover:opacity-75"
			onclick={() => settingsStore.resetChanges()}
		>
			<div class="flex items-center justify-center gap-2">
				<Icon src={ArrowUturnLeft} class="dark:text-mofu-dark-200 h-4 w-4" />
				<span class="dark:text-mofu-dark-200 text-sm">{m.settings_reset_changes()}</span>
			</div>
		</Button>
	{/if}
</div>

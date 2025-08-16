<script lang="ts">
	import {
		CheckCircle,
		ArrowUturnLeft,
		Icon
	} from 'svelte-hero-icons';
	import { authStore } from '$lib/stores/auth.svelte';
	import { settingsStore } from '$lib/stores/settings.svelte';
	import { PersonalInfoSettings } from '../forms/PersonalInfoSettings';
	import AccountSettings from '../forms/AccountSettings.svelte';
	import DisplaySettings from '../forms/DisplaySettings.svelte';
	import WritingSettings from '../forms/WritingSettings.svelte';
	import PrivacySettings from '../forms/PrivacySettings.svelte';
	import NotificationSettings from '../forms/NotificationSettings.svelte';
	import * as Accordion from '$lib/components/ui/accordion';
	import { Button } from '$lib/components/ui/button';
	import * as m from '../../../../paraglide/messages';

	type Props = {
		sections: Array<{
			id: string;
			label: () => string;
			icon: any;
			description: () => string;
			requiresAuth: boolean;
		}>;
		handleSave: () => Promise<void>;
		saveSuccess: boolean;
	};

	const { sections, handleSave, saveSuccess }: Props = $props();

	// 모바일에서 accordion의 기본 열린 섹션
	let accordionValue = $state(authStore.isAuthenticated ? 'personal' : 'display');
</script>

<!-- 모바일 레이아웃 (Accordion) -->
<div class="text-mofu-dark-200 min-h-screen w-full pb-10 lg:hidden">
	<Accordion.Root type="single" bind:value={accordionValue} class="space-y-2">
		{#each sections.filter((s) => !s.requiresAuth || authStore.isAuthenticated) as section}
			<Accordion.Item value={section.id}>
				<Accordion.Trigger class="flex w-full items-center justify-between text-left">
					<div class="flex items-center gap-3">
						<div class="rounded-lg bg-white/10 p-2">
							<Icon src={section.icon} size="20" solid class="text-mofu" />
						</div>
						<div>
							<h3 class="text-mofu-dark-200 text-md font-bold">{section.label()}</h3>
							<p class="text-xs text-gray-400 dark:text-mofu-dark-300">{section.description()}</p>
						</div>
					</div>
				</Accordion.Trigger>
				<Accordion.Content class="pb-4">
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
<div class="fixed right-4 bottom-4 left-4 z-50 space-y-2 text-mofu-dark-200 lg:hidden dark:text-mofu-dark-200">
	<!-- Save Button -->
	<Button
		class="group flex w-full cursor-pointer flex-col overflow-hidden rounded-xl border bg-mofu-dark-800 px-6 py-6 text-left transition-all duration-200 hover:opacity-75 hover:shadow-xl dark:bg-mofu-dark-800 {!settingsStore.hasChanges
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
				<h3 class="text-md font-bold text-mofu-dark-200">{m.settings_saving()}</h3>
			{:else if saveSuccess}
				<Icon src={CheckCircle} class="h-4 w-4 text-green-400" />
				<h3 class="text-md font-bold text-mofu-dark-200">{m.settings_saved()}</h3>
			{:else}
				<h3 class="text-md font-bold text-mofu-dark-200">{m.settings_save_changes()}</h3>
				{#if settingsStore.hasChanges}
					<span class="text-xs text-orange-400">•</span>
				{/if}
			{/if}
		</div>
	</Button>

	<!-- Reset Button (only show if there are changes) -->
	{#if settingsStore.hasChanges}
		<Button
			class="group flex w-full cursor-pointer flex-col overflow-hidden rounded-xl border bg-mofu-dark-800 px-6 py-6 text-center transition-all duration-200 hover:opacity-75 dark:bg-mofu-dark-800"
			onclick={() => settingsStore.resetChanges()}
		>
			<div class="flex items-center justify-center gap-2">
				<Icon src={ArrowUturnLeft} class="h-4 w-4 text-mofu-dark-200 dark:text-mofu-dark-200" />
				<span class="text-sm text-mofu-dark-200 dark:text-mofu-dark-200">{m.settings_reset_changes()}</span>
			</div>
		</Button>
	{/if}
</div>
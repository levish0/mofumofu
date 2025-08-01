<script lang="ts">
	import PersonalInfoSettings from '$lib/components/settings/PersonalInfoSettings.svelte';
	import DisplaySettings from '$lib/components/settings/DisplaySettings.svelte';
	import AccountSettings from '$lib/components/settings/AccountSettings.svelte';
	import WritingSettings from '$lib/components/settings/WritingSettings.svelte';
	import NotificationSettings from '$lib/components/settings/NotificationSettings.svelte';
	import PrivacySettings from '$lib/components/settings/PrivacySettings.svelte';
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
	import { getContext } from 'svelte';

	let selectedSection = $state('personal');

	type NavbarContext = {
		isVisible: () => boolean;
		isAtTop: () => boolean;
	};

	const navbar = getContext<NavbarContext>('navbar');

	// Calculate the top position based on navbar state
	const topPosition = $derived(navbar.isVisible() ? '68px' : '8px');

	function handleSave() {
		console.log('Save Changes');
	}

	const sections = [
		{ id: 'personal', label: 'Personal Info', icon: User, description: 'Manage your profile information' },
		{ id: 'account', label: 'Account', icon: CreditCard, description: 'Manage account settings and billing' },
		{ id: 'display', label: 'Display', icon: ComputerDesktop, description: 'Customize your display preferences' },
		{
			id: 'writing',
			label: 'Writing & Publishing',
			icon: PencilSquare,
			description: 'Configure writing and publishing preferences'
		},
		{ id: 'notifications', label: 'Notifications', icon: Bell, description: 'Control notification settings' },
		{
			id: 'privacy',
			label: 'Privacy & Security',
			icon: ShieldExclamation,
			description: 'Manage privacy and security options'
		}
	];
</script>

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
							: ''}"
						onclick={() => (selectedSection = section.id)}
					>
						<div class="mb-3 flex items-center gap-3">
							<div class="rounded-lg bg-white/10 p-2">
								<Icon src={section.icon} size="20" solid class="text-mofu" />
							</div>
							<div class="flex-1">
								<h3 class="text-mofu-dark-200 text-md font-bold">{section.label}</h3>
							</div>
						</div>
						<p class="text-xs text-gray-400">{section.description}</p>
					</button>
				{/each}
			</div>

			<!-- Save Button -->
			<button
				class="dark:bg-mofu-dark-800 dark:border-mofu-dark-800 group flex w-full cursor-pointer flex-col overflow-hidden rounded-xl border p-4 text-left transition-all duration-200 hover:opacity-75 hover:shadow-xl"
				onclick={handleSave}
			>
				<div class="flex items-center justify-center">
					<h3 class="text-mofu-dark-200 text-md font-bold">Save Changes</h3>
				</div>
			</button>
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

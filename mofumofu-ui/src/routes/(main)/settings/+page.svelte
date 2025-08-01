<script lang="ts">
	import PersonalInfoSettings from '$lib/components/settings/PersonalInfoSettings.svelte';
	import DisplaySettings from '$lib/components/settings/DisplaySettings.svelte';
	import { User, ComputerDesktop, Bell, ShieldExclamation, Icon } from 'svelte-hero-icons';
	import { getContext } from 'svelte';

	let selectedSection = $state('personal');

	type NavbarContext = {
		isVisible: () => boolean;
		isAtTop: () => boolean;
	};

	const navbar = getContext<NavbarContext>('navbar');

	// Calculate the top position based on navbar state
	const topPosition = $derived(navbar.isVisible() ? '68px' : '8px');

	const sections = [
		{ id: 'personal', label: 'Personal Info', icon: User, description: 'Manage your profile information' },
		{ id: 'display', label: 'Display', icon: ComputerDesktop, description: 'Customize your display preferences' },
		{ id: 'notifications', label: 'Notifications', icon: Bell, description: 'Control notification settings' },
		{
			id: 'privacy',
			label: 'Privacy & Security',
			icon: ShieldExclamation,
			description: 'Manage privacy and security options'
		}
	];
</script>

<div class="text-mofu-dark-200 flex min-h-screen w-full gap-6">
	<!-- Sidebar with Card Grid -->
	<div class="w-1/3">
		<div
			class="sticky grid grid-cols-1 gap-3 transition-all duration-100 ease-out sm:grid-cols-2"
			style="top: {topPosition}"
		>
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
							<h3 class="text-sm font-semibold text-gray-100">{section.label}</h3>
						</div>
					</div>
					<p class="text-xs text-gray-400">{section.description}</p>
				</button>
			{/each}
		</div>
	</div>

	<!-- Main Content -->
	<div class="flex-1">
		{#if selectedSection === 'personal'}
			<PersonalInfoSettings />
		{:else if selectedSection === 'display'}
			<DisplaySettings />
		{:else if selectedSection === 'notifications'}
			<div class="rounded-xl border border-slate-700 bg-slate-800 p-8 text-center">
				<h2 class="mb-2 text-xl font-semibold">Notifications</h2>
				<p class="text-gray-400">Notification settings coming soon...</p>
			</div>
		{:else if selectedSection === 'privacy'}
			<div class="rounded-xl border border-slate-700 bg-slate-800 p-8 text-center">
				<h2 class="mb-2 text-xl font-semibold">Privacy & Security</h2>
				<p class="text-gray-400">Privacy and security settings coming soon...</p>
			</div>
		{/if}
	</div>
</div>

<script lang="ts">
	import PersonalInfoSettings from '$lib/components/settings/PersonalInfoSettings.svelte';
	import DisplaySettings from '$lib/components/settings/DisplaySettings.svelte';
	import { User, ComputerDesktop,  Icon } from 'svelte-hero-icons';

	let selectedSection = $state('personal');

	const sections = [
		{ id: 'personal', label: 'Personal Info', icon: User },
		{ id: 'display', label: 'Display', icon: ComputerDesktop }
	];
</script>

<div class="text-mofu-dark-200 flex min-h-screen gap-6">
	<!-- Sidebar -->
	<div class="w-80">
		<div class="space-y-1">
			{#each sections as section}
				<button
					class="  text-md hover:bg-muted flex w-full items-center gap-3 rounded-lg px-4 py-3 text-left font-bold transition-colors hover:bg-white/5 {selectedSection ===
					section.id
						? ' bg-white/5'
						: ''}"
					onclick={() => (selectedSection = section.id)}
				>
					<Icon src={section.icon} size="20" solid class="" />
					<span class="">{section.label}</span>
					<span class="ml-auto">→</span>
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
		{/if}
	</div>
</div>

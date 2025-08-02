<script lang="ts">
	import { Input } from '../ui/input';
	import { Button } from '../ui/button';
	import { Camera, Photo, Icon, Plus } from 'svelte-hero-icons';
	import { z } from 'zod';
	import { settingsStore } from '$lib/stores/settings.svelte.js';

	const handleSchema = z
		.string()
		.min(1, 'Handle is required')
		.min(3, 'Handle must be at least 3 characters')
		.max(20, 'Handle cannot exceed 20 characters')
		.regex(/^[a-zA-Z0-9_]+$/, 'Handle can only contain letters, numbers, and underscores');

	const nameSchema = z
		.string()
		.min(1, 'Display name is required')
		.min(3, 'Display name must be at least 3 characters')
		.max(20, 'Display name cannot exceed 20 characters');

	// Use store values instead of local state
	const personal = $derived(settingsStore.personal);
	const { handle, name, profileImage, bannerImage, profileImageFile, bannerImageFile } = $derived(personal);

	let localErrors = $state<{ handle?: string; name?: string }>({});
	let isCheckingHandle = $state(false);
	let handleAvailable = $state<boolean | null>(null);

	function handleProfileImageChange(event: Event) {
		const target = event.target as HTMLInputElement;
		const file = target.files?.[0];
		if (file) {
			const reader = new FileReader();
			reader.onload = (e) => {
				settingsStore.updatePersonal({
					profileImageFile: file,
					profileImage: e.target?.result as string
				});
			};
			reader.readAsDataURL(file);
		}
		// Reset input to allow selecting the same file again
		target.value = '';
	}

	function handleBannerImageChange(event: Event) {
		const target = event.target as HTMLInputElement;
		const file = target.files?.[0];
		if (file) {
			const reader = new FileReader();
			reader.onload = (e) => {
				settingsStore.updatePersonal({
					bannerImageFile: file,
					bannerImage: e.target?.result as string
				});
			};
			reader.readAsDataURL(file);
		}
		// Reset input to allow selecting the same file again
		target.value = '';
	}

	function validateHandle(value: string): string | undefined {
		const result = handleSchema.safeParse(value.trim());
		return result.success ? undefined : result.error.issues[0]?.message;
	}

	function validateName(value: string): string | undefined {
		const result = nameSchema.safeParse(value.trim());
		return result.success ? undefined : result.error.issues[0]?.message;
	}

	function validateForm(): boolean {
		const newErrors: { handle?: string; name?: string } = {};

		const handleError = validateHandle(handle);
		if (handleError) newErrors.handle = handleError;

		const nameError = validateName(name);
		if (nameError) newErrors.name = nameError;

		localErrors = newErrors;
		return Object.keys(newErrors).length === 0;
	}

	async function checkHandleAvailability() {
		if (!handle.trim() || validateHandle(handle)) {
			return;
		}

		isCheckingHandle = true;
		handleAvailable = null;

		// Simulate API call
		await new Promise((resolve) => setTimeout(resolve, 1000));

		// Mock availability check (in real app, this would be an API call)
		const unavailableHandles = ['admin', 'test', 'user', 'mofu'];
		handleAvailable = !unavailableHandles.includes(handle.toLowerCase());

		isCheckingHandle = false;
	}
</script>

<div class="text-mofu-dark-200 min-h-screen">
	<div class=" space-y-4">
		<!-- Banner Image Section -->
		<div class="space-y-4">
			<h2 class="text-2xl font-semibold">Banner Image</h2>
			<div class="relative">
				<div class="bg-mofu-dark-800 group aspect-[4/1] w-full overflow-hidden rounded-lg">
					{#if bannerImage}
						<img src={bannerImage} alt="Banner preview" class="h-full w-full object-cover" />
						<label
							for="banner-upload"
							class="absolute inset-0 flex cursor-pointer items-center justify-center text-gray-400 hover:text-gray-300"
						>
							<Icon src={Camera} class="h-8 w-8 opacity-0 transition-opacity group-hover:opacity-100" />
						</label>
					{:else}
						<label
							for="banner-upload"
							class="text-mofu-dark-300 flex h-full cursor-pointer flex-col items-center justify-center space-y-2"
						>
							<Icon src={Photo} class="h-10 w-10" />
							<span class="text-sm">Click to upload banner image</span>
							<span class="text-xs">Recommended: 1600x400px</span>
						</label>
					{/if}
				</div>
				<input id="banner-upload" type="file" accept="image/*" class="hidden" onchange={handleBannerImageChange} />
			</div>
		</div>

		<!-- Profile Image Section -->
		<div class="space-y-4">
			<h2 class="text-2xl font-semibold">Profile Image</h2>
			<div class="flex items-center space-x-4">
				<div class="relative">
					<div class="bg-mofu-dark-800 group h-24 w-24 overflow-hidden rounded-full">
						{#if profileImage}
							<img src={profileImage} alt="Profile preview" class="h-full w-full object-cover" />
							<label
								for="profile-upload"
								class="absolute inset-0 flex cursor-pointer items-center justify-center text-gray-400 hover:text-gray-300"
							>
								<Icon src={Camera} class="h-6 w-6 opacity-0 transition-opacity group-hover:opacity-100" />
							</label>
						{:else}
							<label
								for="profile-upload"
								class="flex h-full cursor-pointer items-center justify-center text-gray-400 hover:text-gray-300"
							>
								<Icon src={Camera} class="h-6 w-6" />
							</label>
						{/if}
					</div>
					<input id="profile-upload" type="file" accept="image/*" class="hidden" onchange={handleProfileImageChange} />
				</div>
				<div class="text-sm text-gray-400">
					<p>Recommended: 400x400px</p>
					<p>Max file size: 5MB</p>
				</div>
			</div>
		</div>

		<!-- Handle Section -->
		<div class="space-y-4">
			<h2 class="text-xl font-semibold">Handle</h2>
			<div class="space-y-2">
				<div class="flex">
					<span class="dark:bg-mofu-dark-800/50 text-mofu-dark-200 inline-flex items-center rounded-l-md px-3 text-sm"
						>@</span
					>
					<Input
						id="handle"
						placeholder="username"
						class="dark:bg-mofu-dark-800 text-mofu-dark-200 placeholder:text-mofu-dark-300 rounded-l-none rounded-r-none border-r-0 {localErrors.handle
							? 'border-red-500'
							: ''}"
						value={handle}
						oninput={(e) => {
							const value = (e.target as HTMLInputElement).value;
							settingsStore.updatePersonal({ handle: value });
							if (localErrors.handle) localErrors.handle = undefined;
							handleAvailable = null;
						}}
					/>
					<button
						onclick={checkHandleAvailability}
						disabled={isCheckingHandle || !handle.trim() || !!validateHandle(handle)}
						class="dark:bg-mofu-dark-800/50 text-mofu-dark-200 disabled:dark:bg-mofu-dark-800/50 hover:bg-mofu-dark-700 inline-flex min-w-20 items-center justify-center rounded-r-md px-3 py-2 text-sm font-medium disabled:cursor-not-allowed disabled:opacity-50"
					>
						{#if isCheckingHandle}
							<svg class="h-4 w-4 animate-spin" fill="none" viewBox="0 0 24 24">
								<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
								<path
									class="opacity-75"
									fill="currentColor"
									d="m4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
								></path>
							</svg>
						{:else}
							Check
						{/if}
					</button>
				</div>
				{#if localErrors.handle}
					<p class="text-xs text-red-400">{localErrors.handle}</p>
				{:else if handleAvailable === true}
					<p class="text-xs text-green-400">✓ Handle is available</p>
				{:else if handleAvailable === false}
					<p class="text-xs text-red-400">✗ Handle is already taken</p>
				{:else}
					<p class="text-xs text-gray-500">
						Handle must be unique and can only contain letters, numbers, and underscores.
					</p>
				{/if}
			</div>
		</div>

		<!-- Name Section -->
		<div class="space-y-4 border-t border-slate-700 pt-8">
			<h2 class="text-xl font-semibold">Display Name</h2>
			<p class="text-sm text-gray-400">Your name as it appears on your profile.</p>
			<div class="space-y-2">
				<Input
					id="name"
					placeholder="Enter your display name"
					class="dark:bg-mofu-dark-800 text-mofu-dark-200 placeholder:text-mofu-dark-300 {localErrors.name
						? 'border-red-500'
						: ''}"
					value={name}
					oninput={(e) => {
						const value = (e.target as HTMLInputElement).value;
						settingsStore.updatePersonal({ name: value });
						if (localErrors.name) localErrors.name = undefined;
					}}
				/>
				{#if localErrors.name}
					<p class="text-xs text-red-400">{localErrors.name}</p>
				{:else}
					<p class="text-xs text-gray-500">Display name must be between 3-20 characters.</p>
				{/if}
			</div>
		</div>
	</div>
</div>

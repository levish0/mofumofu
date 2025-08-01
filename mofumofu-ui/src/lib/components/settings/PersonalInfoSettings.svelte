<script lang="ts">
	import { Input } from '../ui/input';
	import { Button } from '../ui/button';
	import { Camera, Photo, Icon, Plus } from 'svelte-hero-icons';

	let handle = $state('');
	let name = $state('');
	let profileImage = $state<string | null>(null);
	let bannerImage = $state<string | null>(null);
	let profileImageFile = $state<File | null>(null);
	let bannerImageFile = $state<File | null>(null);
	let errors = $state<{ handle?: string; name?: string }>({});

	function handleProfileImageChange(event: Event) {
		const target = event.target as HTMLInputElement;
		const file = target.files?.[0];
		if (file) {
			profileImageFile = file;
			const reader = new FileReader();
			reader.onload = (e) => {
				profileImage = e.target?.result as string;
			};
			reader.readAsDataURL(file);
		}
	}

	function handleBannerImageChange(event: Event) {
		const target = event.target as HTMLInputElement;
		const file = target.files?.[0];
		if (file) {
			bannerImageFile = file;
			const reader = new FileReader();
			reader.onload = (e) => {
				bannerImage = e.target?.result as string;
			};
			reader.readAsDataURL(file);
		}
	}

	function removeProfileImage() {
		profileImage = null;
		profileImageFile = null;
	}

	function removeBannerImage() {
		bannerImage = null;
		bannerImageFile = null;
	}

	function validateHandle(value: string): string | undefined {
		if (!value.trim()) return 'Handle is required';
		if (!/^[a-zA-Z0-9_]+$/.test(value)) return 'Handle can only contain letters, numbers, and underscores';
		if (value.length < 3) return 'Handle must be at least 3 characters';
		if (value.length > 20) return 'Handle cannot exceed 20 characters';
		return undefined;
	}

	function validateName(value: string): string | undefined {
		if (!value.trim()) return 'Display name is required';
		if (value.length > 50) return 'Display name cannot exceed 50 characters';
		return undefined;
	}

	function validateForm(): boolean {
		const newErrors: { handle?: string; name?: string } = {};

		const handleError = validateHandle(handle);
		if (handleError) newErrors.handle = handleError;

		const nameError = validateName(name);
		if (nameError) newErrors.name = nameError;

		errors = newErrors;
		return Object.keys(newErrors).length === 0;
	}

	function saveProfile() {
		if (!validateForm()) return;

		console.log('Saving profile:', {
			handle: handle.trim(),
			name: name.trim(),
			profileImageFile,
			bannerImageFile
		});
	}
</script>

<div class="text-mofu-dark-200 min-h-screen">
	<div class=" space-y-4">
		<!-- Banner Image Section -->
		<div class="space-y-4">
			<h2 class="text-2xl font-semibold">Banner Image</h2>
			<div class="relative">
				<div class="bg-mofu-dark-800 aspect-[3/1] w-full overflow-hidden rounded-lg">
					{#if bannerImage}
						<img src={bannerImage} alt="Banner preview" class="h-full w-full object-cover" />
						<button
							onclick={removeBannerImage}
							class="absolute top-2 right-2 rounded-full bg-red-600 p-1 text-white hover:bg-red-700"
						>
							<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
							</svg>
						</button>
					{:else}
						<label
							for="banner-upload"
							class="text-mofu-dark-300 flex h-full cursor-pointer flex-col items-center justify-center space-y-2"
						>
							<Icon src={Photo} class="h-10 w-10" />
							<span class="text-sm">Click to upload banner image</span>
							<span class="text-xs">Recommended: 1200x400px</span>
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
					<div class="bg-mofu-dark-800 h-24 w-24 overflow-hidden rounded-full">
						{#if profileImage}
							<img src={profileImage} alt="Profile preview" class="h-full w-full object-cover" />
							<button
								onclick={removeProfileImage}
								class="absolute -top-1 -right-1 rounded-full bg-red-600 p-1 text-white hover:bg-red-700"
							>
								<svg class="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
								</svg>
							</button>
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
			<p class="text-sm text-gray-400">Your unique username that appears in your profile URL.</p>
			<div class="space-y-2">
				<div class="flex">
					<span
						class="inline-flex items-center rounded-l-md border border-r-0 border-slate-700 bg-slate-800 px-3 text-sm text-gray-400"
						>@</span
					>
					<Input
						id="handle"
						placeholder="username"
						class="rounded-l-none border-slate-700 bg-slate-800 text-gray-100 {errors.handle ? 'border-red-500' : ''}"
						bind:value={handle}
						oninput={() => {
							if (errors.handle) errors.handle = undefined;
						}}
					/>
				</div>
				{#if errors.handle}
					<p class="text-xs text-red-400">{errors.handle}</p>
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
					class="w-full border-slate-700 bg-slate-800 text-gray-100 {errors.name ? 'border-red-500' : ''}"
					bind:value={name}
					oninput={() => {
						if (errors.name) errors.name = undefined;
					}}
				/>
				{#if errors.name}
					<p class="text-xs text-red-400">{errors.name}</p>
				{/if}
			</div>
		</div>
	</div>
</div>

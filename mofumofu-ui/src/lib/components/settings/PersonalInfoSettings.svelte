<script lang="ts">
	import { Input } from '../ui/input';
	import { Camera, Photo, Icon, Plus } from 'svelte-hero-icons';
	import * as v from 'valibot';
	import { personalInfoSchema } from '$lib/schemas/personal-info';
	import { settingsStore } from '$lib/stores/settings.svelte.js';
	import { onMount } from 'svelte';
	import ImageCropModal from '../modal/ImageCropModal.svelte';
	import { getCroppedImg } from '$lib/utils/imageCrop';

	// Validation schemas are now imported from schemas/personal-info.ts

	// Use store values instead of local state
	const personal = $derived(settingsStore.personal);
	const { handle, name, profileImage, bannerImage, profileImageFile, bannerImageFile } = $derived(personal);

	let localErrors = $state<{ handle?: string; name?: string }>({});
	let isCheckingHandle = $state(false);
	let handleAvailable = $state<boolean | null>(null);

	// Crop modal states
	let showBannerCrop = $state(false);
	let showProfileCrop = $state(false);
	let tempImageSrc = $state('');

	function handleProfileImageChange(event: Event) {
		const target = event.target as HTMLInputElement;
		const file = target.files?.[0];
		if (file && file.type.startsWith('image/')) {
			const reader = new FileReader();
			reader.onload = (e) => {
				const result = e.target?.result;
				if (typeof result === 'string') {
					tempImageSrc = result;
					showProfileCrop = true;
				}
			};
			reader.onerror = () => {
				console.error('Failed to read image file');
				alert('Failed to read image file. Please try again.');
			};
			reader.readAsDataURL(file);
		}
		// Reset input to allow selecting the same file again
		target.value = '';
	}

	function handleBannerImageChange(event: Event) {
		const target = event.target as HTMLInputElement;
		const file = target.files?.[0];
		if (file && file.type.startsWith('image/')) {
			const reader = new FileReader();
			reader.onload = (e) => {
				const result = e.target?.result;
				if (typeof result === 'string') {
					tempImageSrc = result;
					showBannerCrop = true;
				}
			};
			reader.onerror = () => {
				console.error('Failed to read image file');
				alert('Failed to read image file. Please try again.');
			};
			reader.readAsDataURL(file);
		}
		// Reset input to allow selecting the same file again
		target.value = '';
	}

	function validateHandle(value: string): string | undefined {
		const result = v.safeParse(personalInfoSchema.entries.handle, value.trim());
		return result.success ? undefined : result.issues?.[0]?.message;
	}

	function validateName(value: string): string | undefined {
		const result = v.safeParse(personalInfoSchema.entries.name, value.trim());
		return result.success ? undefined : result.issues?.[0]?.message;
	}

	function validateForm(): boolean {
		const newErrors: { handle?: string; name?: string } = {};

		const handleError = validateHandle(handle);
		if (handleError) newErrors.handle = handleError;

		const nameError = validateName(name);
		if (nameError) newErrors.name = nameError;

		localErrors = newErrors;

		// Also update the settings store validation errors
		if (Object.keys(newErrors).length > 0) {
			settingsStore.setValidationErrors('personal', newErrors);
		} else {
			settingsStore.clearValidationErrors('personal');
		}

		return Object.keys(newErrors).length === 0;
	}

	async function handleBannerCrop(data: { croppedAreaPixels: any; croppedAreaPercentage: any }) {
		try {
			const { blob, url } = await getCroppedImg(
				tempImageSrc,
				data.croppedAreaPixels,
				{ width: 1000, height: 250 }, // Standard banner size 4:1 ratio
				0.9 // Quality set to 80%
			);

			// Clean up old banner image URL if it exists
			if (bannerImage && bannerImage.startsWith('blob:')) {
				URL.revokeObjectURL(bannerImage);
			}

			settingsStore.updatePersonal({
				bannerImageFile: blob,
				bannerImage: url
			});
			tempImageSrc = '';
		} catch (error) {
			console.error('Error cropping banner image:', error);
		}
	}

	async function handleProfileCrop(data: { croppedAreaPixels: any; croppedAreaPercentage: any }) {
		try {
			const { blob, url } = await getCroppedImg(
				tempImageSrc,
				data.croppedAreaPixels,
				{ width: 400, height: 400 }, // Standard profile image size 1:1 ratio
				0.9 // Quality set to 80%
			);

			// Clean up old profile image URL if it exists
			if (profileImage && profileImage.startsWith('blob:')) {
				URL.revokeObjectURL(profileImage);
			}

			settingsStore.updatePersonal({
				profileImageFile: blob,
				profileImage: url
			});
			tempImageSrc = '';
		} catch (error) {
			console.error('Error cropping profile image:', error);
		}
	}

	function handleCropCancel() {
		// Clean up temp image URL
		if (tempImageSrc && tempImageSrc.startsWith('blob:')) {
			URL.revokeObjectURL(tempImageSrc);
		}
		tempImageSrc = '';
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

	// Initialize validation after data is loaded
	onMount(() => {
		// Wait for data to be initialized before validating
		if (handle && name) {
			validateForm();
		}
	});
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
			<h2 class="text-2xl font-semibold">Handle</h2>
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
							// Trigger validation
							validateForm();
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
					<p class="text-xs text-rose-400">{localErrors.handle}</p>
				{:else if handleAvailable === true}
					<p class="text-xs text-green-400">✓ Handle is available</p>
				{:else if handleAvailable === false}
					<p class="text-xs text-rose-400">✗ Handle is already taken</p>
				{:else}
					<p class="text-xs text-gray-500">
						Handle must be unique and can only contain letters, numbers, and underscores.
					</p>
				{/if}
			</div>
		</div>

		<!-- Name Section -->
		<div class="space-y-4">
			<h2 class="text-2xl font-semibold">Display Name</h2>
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
						// Trigger validation
						validateForm();
					}}
				/>
				{#if localErrors.name}
					<p class="text-xs text-rose-400">{localErrors.name}</p>
				{:else}
					<p class="text-mofu-dark-300 text-xs">Display name must be between 3-20 characters.</p>
				{/if}
			</div>
		</div>
	</div>

	<!-- Banner Crop Modal -->
	<ImageCropModal
		bind:isOpen={showBannerCrop}
		imageSrc={tempImageSrc}
		aspectRatio={4}
		cropShape="rect"
		onCrop={handleBannerCrop}
		onCancel={handleCropCancel}
	/>

	<!-- Profile Crop Modal -->
	<ImageCropModal
		bind:isOpen={showProfileCrop}
		imageSrc={tempImageSrc}
		aspectRatio={1}
		cropShape="round"
		onCrop={handleProfileCrop}
		onCancel={handleCropCancel}
	/>
</div>

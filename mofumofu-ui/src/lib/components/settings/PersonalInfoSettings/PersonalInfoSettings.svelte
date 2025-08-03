<script lang="ts">
	import { settingsStore } from '$lib/stores/settings.svelte';
	import { onMount } from 'svelte';
	import BannerImageUpload from './BannerImageUpload.svelte';
	import ProfileImageUpload from './ProfileImageUpload.svelte';
	import HandleInput from './HandleInput.svelte';
	import DisplayNameInput from './DisplayNameInput.svelte';

	const personal = $derived(settingsStore.personal);
	const { handle, name, profileImage, bannerImage } = $derived(personal);

	let localErrors = $state<{ handle?: string; name?: string }>({});

	function validateForm(): boolean {
		const hasErrors = Object.keys(localErrors).length > 0;

		if (hasErrors) {
			settingsStore.setValidationErrors('personal', localErrors);
		} else {
			settingsStore.clearValidationErrors('personal');
		}

		return !hasErrors;
	}

	function handleBannerUpdate(data: { bannerImageFile: Blob; bannerImage: string }) {
		settingsStore.updatePersonal(data);
	}

	function handleProfileUpdate(data: { profileImageFile: Blob; profileImage: string }) {
		settingsStore.updatePersonal(data);
	}

	function handleHandleUpdate(newHandle: string) {
		settingsStore.updatePersonal({ handle: newHandle });
	}

	function handleNameUpdate(newName: string) {
		settingsStore.updatePersonal({ name: newName });
	}

	function handleHandleValidation(error?: string) {
		if (error) {
			localErrors.handle = error;
		} else {
			delete localErrors.handle;
		}
		localErrors = { ...localErrors };
		validateForm();
	}

	function handleNameValidation(error?: string) {
		if (error) {
			localErrors.name = error;
		} else {
			delete localErrors.name;
		}
		localErrors = { ...localErrors };
		validateForm();
	}

	onMount(() => {
		if (handle && name) {
			validateForm();
		}
	});
</script>

<div class="text-mofu-dark-200 min-h-screen">
	<div class="space-y-4">
		<BannerImageUpload {bannerImage} onUpdate={handleBannerUpdate} />

		<ProfileImageUpload {profileImage} onUpdate={handleProfileUpdate} />

		<HandleInput {handle} onUpdate={handleHandleUpdate} onValidationChange={handleHandleValidation} />

		<DisplayNameInput {name} onUpdate={handleNameUpdate} onValidationChange={handleNameValidation} />
	</div>
</div>

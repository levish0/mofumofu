<script lang="ts">
	import { settingsStore } from '$lib/stores/settings.svelte';
	import { onMount } from 'svelte';
	import BannerImageUpload from './BannerImageUpload.svelte';
	import ProfileImageUpload from './ProfileImageUpload.svelte';
	import HandleInput from './HandleInput.svelte';
	import DisplayNameInput from './DisplayNameInput.svelte';
	import BioInput from './BioInput.svelte';
	import LocationInput from './LocationInput.svelte';
	import WebsiteInput from './WebsiteInput.svelte';

	const personal = $derived(settingsStore.personal);
	const { handle, name, profileImage, bannerImage, bio, location, website } = $derived(personal);

	let localErrors = $state<{ handle?: string; name?: string; bio?: string; location?: string; website?: string }>({});

	// 스토어에서 검증을 트리거하는 간단한 함수
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
		validateForm();
	}

	function handleNameUpdate(newName: string) {
		settingsStore.updatePersonal({ name: newName });
	}

	function handleBioUpdate(newBio: string) {
		settingsStore.updatePersonal({ bio: newBio });
	}

	function handleLocationUpdate(newLocation: string) {
		settingsStore.updatePersonal({ location: newLocation });
	}

	function handleWebsiteUpdate(newWebsite: string) {
		settingsStore.updatePersonal({ website: newWebsite });
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

	function handleBioValidation(error?: string) {
		if (error) {
			localErrors.bio = error;
		} else {
			delete localErrors.bio;
		}
		localErrors = { ...localErrors };
		validateForm();
	}

	function handleLocationValidation(error?: string) {
		if (error) {
			localErrors.location = error;
		} else {
			delete localErrors.location;
		}
		localErrors = { ...localErrors };
		validateForm();
	}

	function handleWebsiteValidation(error?: string) {
		if (error) {
			localErrors.website = error;
		} else {
			delete localErrors.website;
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

		<BioInput {bio} onUpdate={handleBioUpdate} onValidationChange={handleBioValidation} />

		<LocationInput {location} onUpdate={handleLocationUpdate} onValidationChange={handleLocationValidation} />

		<WebsiteInput {website} onUpdate={handleWebsiteUpdate} onValidationChange={handleWebsiteValidation} />
	</div>
</div>

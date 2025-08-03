import { updateProfile, uploadAvatar, uploadBanner, getMyProfile } from '$lib/api/user/userApi';
import type { UpdateProfileRequest } from '$lib/api/user/types';
import { personalInfoSchema } from '$lib/schemas/personal-info';
import { safeParse } from 'valibot';
import type { PersonalInfo } from './types';

export class PersonalSettingsStore {
	private state = $state<PersonalInfo>({
		handle: '',
		name: '',
		profileImage: null,
		bannerImage: null,
		profileImageFile: null,
		bannerImageFile: null
	});

	private originalState = $state<PersonalInfo>({
		handle: '',
		name: '',
		profileImage: null,
		bannerImage: null,
		profileImageFile: null,
		bannerImageFile: null
	});

	private errors = $state<Record<string, string>>({});
	private validationErrors = $state<Record<string, string>>({});

	// Getters
	get data() {
		return this.state;
	}

	get handle() {
		return this.state.handle;
	}

	get name() {
		return this.state.name;
	}

	get profileImage() {
		return this.state.profileImage;
	}

	get bannerImage() {
		return this.state.bannerImage;
	}

	get profileImageFile() {
		return this.state.profileImageFile;
	}

	get bannerImageFile() {
		return this.state.bannerImageFile;
	}

	get hasChanges() {
		const current = JSON.stringify({
			handle: this.state.handle,
			name: this.state.name,
			profileImage: this.state.profileImage,
			bannerImage: this.state.bannerImage
		});
		const original = JSON.stringify({
			handle: this.originalState.handle,
			name: this.originalState.name,
			profileImage: this.originalState.profileImage,
			bannerImage: this.originalState.bannerImage
		});
		return current !== original || this.state.profileImageFile !== null || this.state.bannerImageFile !== null;
	}

	get currentErrors() {
		return this.errors;
	}

	get currentValidationErrors() {
		return this.validationErrors;
	}

	// Initialize with server data
	initialize(data: Partial<PersonalInfo>) {
		this.state = { ...this.state, ...data };
		this.originalState = JSON.parse(JSON.stringify(this.state));
	}

	// Initialize with defaults (called from main settings store)
	initializeWithDefaults() {
		this.originalState = JSON.parse(JSON.stringify(this.state));
	}

	// Update without triggering change detection (for initial load)
	updateSilent(updates: Partial<PersonalInfo>) {
		this.state = { ...this.state, ...updates };
		// Update original state too so it doesn't count as a change
		this.originalState = { ...this.originalState, ...updates };
	}

	// Update with change detection
	update(updates: Partial<PersonalInfo>) {
		this.state = { ...this.state, ...updates };
	}

	// Validate personal info
	async validate(): Promise<boolean> {
		const result = safeParse(personalInfoSchema, this.state);
		if (!result.success) {
			const errors: Record<string, string> = {};
			for (const issue of result.issues) {
				if (issue.path && issue.path.length > 0) {
					const field = issue.path[0].key as string;
					errors[field] = issue.message;
				}
			}
			this.validationErrors = errors;
			return false;
		} else {
			this.validationErrors = {};
			return true;
		}
	}

	// Check if personal info has changes including files
	private hasPersonalChanges(): boolean {
		const current = this.state;
		const original = this.originalState;

		return (
			current.handle !== original.handle ||
			current.name !== original.name ||
			current.profileImage !== original.profileImage ||
			current.bannerImage !== original.bannerImage ||
			current.profileImageFile !== null ||
			current.bannerImageFile !== null
		);
	}

	// Save changes
	async save(accountPassword?: string): Promise<{ success: boolean; error?: any }> {
		try {
			// Validate before saving
			const isValid = await this.validate();
			if (!isValid) {
				this.errors = { general: 'Please fix validation errors before saving.' };
				return { success: false, error: 'Validation failed' };
			}

			if (this.hasPersonalChanges() || (accountPassword && accountPassword.trim() !== '')) {
				// Upload profile image if file exists
				if (this.state.profileImageFile) {
					try {
						await uploadAvatar(this.state.profileImageFile as File);
					} catch (error) {
						console.error('Profile image upload failed:', error);
						this.errors = { general: 'Failed to upload profile image.' };
						return { success: false, error: 'Profile image upload failed' };
					}
				}

				// Upload banner image if file exists
				if (this.state.bannerImageFile) {
					try {
						await uploadBanner(this.state.bannerImageFile as File);
					} catch (error) {
						console.error('Banner image upload failed:', error);
						this.errors = { general: 'Failed to upload banner image.' };
						return { success: false, error: 'Banner image upload failed' };
					}
				}

				// Update basic profile info (handle, name, password)
				const personalData: UpdateProfileRequest = {
					handle: this.state.handle || null,
					name: this.state.name || null
				};

				// Add password if provided
				if (accountPassword && accountPassword.trim() !== '') {
					personalData.password = accountPassword;
				}

				await updateProfile(personalData);

				// Fetch updated profile to get latest image URLs after upload
				const updatedProfile = await getMyProfile();

				// Update the personal info with the response from the API
				// Keep the current blob URLs if they exist, otherwise use server URLs
				this.state = {
					...this.state,
					handle: updatedProfile.handle,
					name: updatedProfile.name,
					profileImage: this.state.profileImage || updatedProfile.profile_image || null,
					bannerImage: this.state.bannerImage || updatedProfile.banner_image || null,
					profileImageFile: null, // Clear file after successful upload
					bannerImageFile: null // Clear file after successful upload
				};

				// Update original state
				this.originalState = JSON.parse(JSON.stringify(this.state));
				this.errors = {};
			}

			return { success: true };
		} catch (error) {
			console.error('Failed to save personal settings:', error);
			this.errors = { general: 'Failed to save personal settings. Please try again.' };
			return { success: false, error };
		}
	}

	// Reset to original state
	reset() {
		// Clean up blob URLs before resetting
		if (this.state.profileImage && this.state.profileImage.startsWith('blob:')) {
			URL.revokeObjectURL(this.state.profileImage);
		}
		if (this.state.bannerImage && this.state.bannerImage.startsWith('blob:')) {
			URL.revokeObjectURL(this.state.bannerImage);
		}

		this.state = JSON.parse(JSON.stringify(this.originalState));
		this.errors = {};
		this.validationErrors = {};
	}

	// Error management
	setError(field: string, message: string) {
		this.errors = { ...this.errors, [field]: message };
	}

	clearError(field: string) {
		const { [field]: _, ...rest } = this.errors;
		this.errors = rest;
	}

	clearErrors() {
		this.errors = {};
	}

	setValidationErrors(errors: Record<string, string>) {
		this.validationErrors = errors;
	}

	clearValidationErrors() {
		this.validationErrors = {};
	}

	hasValidationErrors(): boolean {
		return Object.keys(this.validationErrors).length > 0;
	}
}

export const personalSettingsStore = new PersonalSettingsStore();
import { updateProfile } from '$lib/api/user/userApi';
import type { UpdateProfileRequest } from '$lib/api/user/types';
import { personalInfoSchema } from '$lib/schemas/personal-info';
import { safeParse } from 'valibot';

export type PersonalInfo = {
	handle: string;
	name: string;
	profileImage: string | null;
	bannerImage: string | null;
	profileImageFile: Blob | null;
	bannerImageFile: Blob | null;
};

export type DisplaySettings = {
	theme: 'light' | 'dark' | 'system';
	fontSize: 'small' | 'medium' | 'large';
	language: string;
};

export type NotificationSettings = {
	// Push notifications
	newPosts: boolean;
	comments: boolean;
	likes: boolean;
	newFollowers: boolean;
	mentions: boolean;
	// Email notifications
	weeklyDigest: boolean;
	securityAlerts: boolean;
	productUpdates: boolean;
	marketing: boolean;
	// Do not disturb
	doNotDisturbEnabled: boolean;
	doNotDisturbStart: string;
	doNotDisturbEnd: string;
	weekendMode: boolean;
};

export type PrivacySettings = {
	profileVisibility: 'public' | 'private';
	searchableByEmail: boolean;
	allowDirectMessages: boolean;
};

export type AccountSettings = {
	email: string;
	password: string;
	twoFactorEnabled: boolean;
};

export type WritingSettings = {
	defaultVisibility: 'public' | 'unlisted' | 'private';
	autoSave: boolean;
	spellCheck: boolean;
};

export type SettingsState = {
	personal: PersonalInfo;
	display: DisplaySettings;
	notifications: NotificationSettings;
	privacy: PrivacySettings;
	account: AccountSettings;
	writing: WritingSettings;
	hasChanges: boolean;
	isLoading: boolean;
	errors: Record<string, string>;
	validationErrors: Record<string, Record<string, string>>;
};

class SettingsStore {
	private state = $state<SettingsState>({
		personal: {
			handle: '',
			name: '',
			profileImage: null,
			bannerImage: null,
			profileImageFile: null,
			bannerImageFile: null
		},
		display: {
			theme: 'system',
			fontSize: 'medium',
			language: 'en'
		},
		notifications: {
			newPosts: true,
			comments: true,
			likes: false,
			newFollowers: true,
			mentions: true,
			weeklyDigest: true,
			securityAlerts: true,
			productUpdates: false,
			marketing: false,
			doNotDisturbEnabled: false,
			doNotDisturbStart: '22:00',
			doNotDisturbEnd: '08:00',
			weekendMode: false
		},
		privacy: {
			profileVisibility: 'public',
			searchableByEmail: true,
			allowDirectMessages: true
		},
		account: {
			email: '',
			password: '',
			twoFactorEnabled: false
		},
		writing: {
			defaultVisibility: 'public',
			autoSave: true,
			spellCheck: true
		},
		hasChanges: false,
		isLoading: false,
		errors: {},
		validationErrors: {}
	});

	private originalState = $state<Partial<SettingsState>>({});

	// Getters
	get personal() {
		return this.state.personal;
	}
	get display() {
		return this.state.display;
	}
	get notifications() {
		return this.state.notifications;
	}
	get privacy() {
		return this.state.privacy;
	}
	get account() {
		return this.state.account;
	}
	get writing() {
		return this.state.writing;
	}
	get hasChanges() {
		return this.state.hasChanges;
	}
	get isLoading() {
		return this.state.isLoading;
	}
	get errors() {
		return this.state.errors;
	}
	get validationErrors() {
		return this.state.validationErrors;
	}

	// Initialize with current user data
	async loadSettings() {
		this.state.isLoading = true;
		try {
			// TODO: API call to load current settings
			// const response = await api.get('/user/settings');
			// this.state = { ...this.state, ...response.data };

			// Store original state for comparison
			this.originalState = JSON.parse(JSON.stringify(this.state));
			this.state.hasChanges = false;
		} catch (error) {
			console.error('Failed to load settings:', error);
		} finally {
			this.state.isLoading = false;
		}
	}

	// Initialize settings with server data
	initializeWithDefaults() {
		this.state.isLoading = false;
		// State is already initialized with defaults, just mark as ready
		this.originalState = JSON.parse(JSON.stringify(this.state));
	}

	initializeSettings(serverSettings: Partial<SettingsState>) {
		this.state.isLoading = true;
		try {
			// Update state with server data, keeping existing structure
			if (serverSettings.personal) {
				this.state.personal = { ...this.state.personal, ...serverSettings.personal };
			}
			if (serverSettings.display) {
				this.state.display = { ...this.state.display, ...serverSettings.display };
			}
			if (serverSettings.notifications) {
				this.state.notifications = { ...this.state.notifications, ...serverSettings.notifications };
			}
			if (serverSettings.privacy) {
				this.state.privacy = { ...this.state.privacy, ...serverSettings.privacy };
			}
			if (serverSettings.account) {
				this.state.account = { ...this.state.account, ...serverSettings.account };
			}
			if (serverSettings.writing) {
				this.state.writing = { ...this.state.writing, ...serverSettings.writing };
			}

			// Store original state for comparison
			this.originalState = JSON.parse(JSON.stringify(this.state));
			this.state.hasChanges = false;
			this.state.errors = {};
			this.state.validationErrors = {};
		} catch (error) {
			console.error('Failed to initialize settings:', error);
		} finally {
			this.state.isLoading = false;
		}
	}

	// Update specific section
	updatePersonal(updates: Partial<PersonalInfo>) {
		this.state.personal = { ...this.state.personal, ...updates };
		this.checkForChanges();
	}

	// Update personal info without triggering change detection (for initial load)
	updatePersonalSilent(updates: Partial<PersonalInfo>) {
		this.state.personal = { ...this.state.personal, ...updates };
		// Update original state too so it doesn't count as a change
		if (this.originalState.personal) {
			this.originalState.personal = { ...this.originalState.personal, ...updates };
		} else {
			this.originalState.personal = { ...this.state.personal };
		}
	}

	updateDisplay(updates: Partial<DisplaySettings>) {
		this.state.display = { ...this.state.display, ...updates };
		this.checkForChanges();
	}

	updateNotifications(updates: Partial<NotificationSettings>) {
		this.state.notifications = { ...this.state.notifications, ...updates };
		this.checkForChanges();
	}

	updatePrivacy(updates: Partial<PrivacySettings>) {
		this.state.privacy = { ...this.state.privacy, ...updates };
		this.checkForChanges();
	}

	updateAccount(updates: Partial<AccountSettings>) {
		this.state.account = { ...this.state.account, ...updates };
		this.checkForChanges();
	}

	updateWriting(updates: Partial<WritingSettings>) {
		this.state.writing = { ...this.state.writing, ...updates };
		this.checkForChanges();
	}

	// Check if current state differs from original
	private checkForChanges() {
		const current = JSON.stringify(this.state);
		const original = JSON.stringify(this.originalState);
		this.state.hasChanges = current !== original;
	}

	// Check if personal info has changes
	private hasPersonalChanges(): boolean {
		const current = this.state.personal;
		const original = this.originalState.personal;
		if (!original) return true;

		return (
			current.handle !== original.handle ||
			current.name !== original.name ||
			current.profileImage !== original.profileImage ||
			current.bannerImage !== original.bannerImage
		);
	}

	// Save all changes
	async saveChanges() {
		this.state.isLoading = true;
		this.state.errors = {};

		try {
			// Validate all sections before saving
			const isValid = await this.validateAll();
			if (!isValid) {
				this.state.errors = { general: 'Please fix validation errors before saving.' };
				return { success: false, error: 'Validation failed' };
			}

			// Save personal info using the profile API
			if (this.hasPersonalChanges()) {
				const personalData: UpdateProfileRequest = {
					handle: this.state.personal.handle || null,
					name: this.state.personal.name || null,
					profile_image: this.state.personal.profileImage,
					banner_image: this.state.personal.bannerImage
				};

				const updatedProfile = await updateProfile(personalData);

				// Update the personal info with the response from the API
				this.state.personal = {
					...this.state.personal,
					handle: updatedProfile.handle,
					name: updatedProfile.name,
					profileImage: updatedProfile.profile_image || null,
					bannerImage: updatedProfile.banner_image || null
				};
			}

			// TODO: API calls to save other sections
			// await api.post('/user/settings/display', this.state.display);
			// await api.post('/user/settings/notifications', this.state.notifications);
			// await api.post('/user/settings/privacy', this.state.privacy);
			// await api.post('/user/settings/account', this.state.account);
			// await api.post('/user/settings/writing', this.state.writing);

			console.log('Settings saved successfully');

			// Update original state to current state
			this.originalState = JSON.parse(JSON.stringify(this.state));
			this.state.hasChanges = false;

			return { success: true };
		} catch (error) {
			console.error('Failed to save settings:', error);
			this.state.errors = { general: 'Failed to save settings. Please try again.' };
			return { success: false, error };
		} finally {
			this.state.isLoading = false;
		}
	}

	// Reset to original state
	resetChanges() {
		if (this.originalState) {
			this.state = { ...this.state, ...JSON.parse(JSON.stringify(this.originalState)) };
			this.state.hasChanges = false;
			this.state.errors = {};
			this.state.validationErrors = {};
		}
	}

	// Set validation errors
	setError(field: string, message: string) {
		this.state.errors = { ...this.state.errors, [field]: message };
	}

	// Clear specific error
	clearError(field: string) {
		const { [field]: _, ...rest } = this.state.errors;
		this.state.errors = rest;
	}

	// Clear all errors
	clearErrors() {
		this.state.errors = {};
	}

	// Set validation errors for a specific section
	setValidationErrors(section: string, errors: Record<string, string>) {
		this.state.validationErrors = {
			...this.state.validationErrors,
			[section]: errors
		};
	}

	// Clear validation errors for a specific section
	clearValidationErrors(section: string) {
		const { [section]: _, ...rest } = this.state.validationErrors;
		this.state.validationErrors = rest;
	}

	// Check if there are any validation errors
	hasValidationErrors(): boolean {
		return Object.values(this.state.validationErrors).some((sectionErrors) => Object.keys(sectionErrors).length > 0);
	}

	// Validate all sections before saving
	async validateAll(): Promise<boolean> {

		let isValid = true;

		// Validate personal info
		const personalResult = safeParse(personalInfoSchema, this.state.personal);
		if (!personalResult.success) {
			const personalErrors: Record<string, string> = {};
			for (const issue of personalResult.issues) {
				if (issue.path && issue.path.length > 0) {
					const field = issue.path[0].key as string;
					personalErrors[field] = issue.message;
				}
			}
			this.setValidationErrors('personal', personalErrors);
			isValid = false;
		} else {
			this.clearValidationErrors('personal');
		}

		// Add other section validations here as needed
		// TODO: Add display, notifications, privacy, account, writing validations

		return isValid;
	}
}

export const settingsStore = new SettingsStore();

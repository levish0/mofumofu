export type PersonalInfo = {
	handle: string;
	name: string;
	profileImage: string | null;
	bannerImage: string | null;
	profileImageFile: File | null;
	bannerImageFile: File | null;
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
		errors: {}
	});

	private originalState = $state<Partial<SettingsState>>({});

	// Getters
	get personal() { return this.state.personal; }
	get display() { return this.state.display; }
	get notifications() { return this.state.notifications; }
	get privacy() { return this.state.privacy; }
	get account() { return this.state.account; }
	get writing() { return this.state.writing; }
	get hasChanges() { return this.state.hasChanges; }
	get isLoading() { return this.state.isLoading; }
	get errors() { return this.state.errors; }

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

	// Update specific section
	updatePersonal(updates: Partial<PersonalInfo>) {
		this.state.personal = { ...this.state.personal, ...updates };
		this.checkForChanges();
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

	// Save all changes
	async saveChanges() {
		this.state.isLoading = true;
		this.state.errors = {};
		
		try {
			// TODO: API calls to save each section
			// await api.post('/user/settings/personal', this.state.personal);
			// await api.post('/user/settings/display', this.state.display);
			// await api.post('/user/settings/notifications', this.state.notifications);
			// await api.post('/user/settings/privacy', this.state.privacy);
			// await api.post('/user/settings/account', this.state.account);
			// await api.post('/user/settings/writing', this.state.writing);

			console.log('Saving settings:', this.state);
			
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
}

export const settingsStore = new SettingsStore();
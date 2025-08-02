import type { PageServerLoad } from './$types';
import type { SettingsState } from '$lib/stores/settings.svelte';

export const load: PageServerLoad = async ({ cookies }) => {
	// TODO: Replace with actual API calls when available
	// const userId = cookies.get('user_id');
	// const settings = await api.getUserSettings(userId);

	// Dummy data that simulates what would come from the API
	const settings: Partial<SettingsState> = {
		personal: {
			handle: 'johndoe',
			name: 'John Doe',
			profileImage: null,
			bannerImage: null,
			profileImageFile: null,
			bannerImageFile: null
		},
		display: {
			theme: 'dark' as const,
			fontSize: 'medium' as const,
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
			profileVisibility: 'public' as const,
			searchableByEmail: true,
			allowDirectMessages: true
		},
		account: {
			email: 'john.doe@example.com',
			password: '',
			twoFactorEnabled: false
		},
		writing: {
			defaultVisibility: 'public' as const,
			autoSave: true,
			spellCheck: true
		}
	};

	return {
		settings
	};
};

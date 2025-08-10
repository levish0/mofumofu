// src/lib/stores/auth.svelte.ts

import { browser } from '$app/environment';
import { userStore } from './user.svelte';

let _token = $state(browser ? (localStorage.getItem('access_token') ?? '') : '');

export const authStore = {
	get token() {
		return _token;
	},

	get isAuthenticated() {
		return _token !== '';
	},

	setToken(token: string) {
		_token = token;
		if (browser) {
			localStorage.setItem('access_token', token);
			if (token) {
				userStore.loadProfile();
			}
		}
	},

	clearToken() {
		_token = '';
		if (browser) {
			localStorage.removeItem('access_token');
			userStore.clear();
		}
	}
};

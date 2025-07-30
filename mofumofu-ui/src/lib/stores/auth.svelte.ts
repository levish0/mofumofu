// src/lib/stores/auth.svelte.ts
// 메모리에만 저장되는 access token
let _token = $state('');

export const authStore = {
	get token() {
		return _token;
	},

	get isAuthenticated() {
		return _token !== '';
	},

	setToken(token: string) {
		_token = token;
	},

	clearToken() {
		_token = '';
	}
};

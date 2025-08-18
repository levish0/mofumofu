// src/lib/oauth/config.ts

import { PUBLIC_GOOGLE_CLIENT_ID, PUBLIC_GITHUB_CLIENT_ID, PUBLIC_APP_URL } from '$env/static/public';

export const OAUTH_CONFIG = {
	google: {
		clientId: PUBLIC_GOOGLE_CLIENT_ID,
		redirectUri: `${PUBLIC_APP_URL}/account/oauth/callback/google`,
		linkRedirectUri: `${PUBLIC_APP_URL}/account/oauth/link/google`,
		scope: 'openid email profile',
		responseType: 'code',
		authUrl: 'https://accounts.google.com/o/oauth2/v2/auth'
	},
	github: {
		clientId: PUBLIC_GITHUB_CLIENT_ID,
		redirectUri: `${PUBLIC_APP_URL}/account/oauth/callback/github`,
		scope: 'user:email',
		authUrl: 'https://github.com/login/oauth/authorize'
	}
};

export function getGoogleOAuthUrl(): string {
	const params = new URLSearchParams({
		client_id: OAUTH_CONFIG.google.clientId,
		redirect_uri: OAUTH_CONFIG.google.redirectUri,
		response_type: OAUTH_CONFIG.google.responseType,
		scope: OAUTH_CONFIG.google.scope,
		access_type: 'offline',
		prompt: 'consent'
	});

	return `${OAUTH_CONFIG.google.authUrl}?${params.toString()}`;
}

export function getGitHubOAuthUrl(): string {
	const params = new URLSearchParams({
		client_id: OAUTH_CONFIG.github.clientId,
		redirect_uri: OAUTH_CONFIG.github.redirectUri,
		scope: OAUTH_CONFIG.github.scope,
		state: Math.random().toString(36).substring(2, 15)
	});

	return `${OAUTH_CONFIG.github.authUrl}?${params.toString()}`;
}

export function getGoogleOAuthLinkUrl(): string {
	const params = new URLSearchParams({
		client_id: OAUTH_CONFIG.google.clientId,
		redirect_uri: OAUTH_CONFIG.google.linkRedirectUri,
		response_type: OAUTH_CONFIG.google.responseType,
		scope: OAUTH_CONFIG.google.scope,
		access_type: 'offline',
		prompt: 'consent'
	});

	return `${OAUTH_CONFIG.google.authUrl}?${params.toString()}`;
}

export function getGitHubOAuthLinkUrl(): string {
	const params = new URLSearchParams({
		client_id: OAUTH_CONFIG.github.clientId,
		redirect_uri: OAUTH_CONFIG.github.redirectUri,
		scope: OAUTH_CONFIG.github.scope,
		state: `link_${Math.random().toString(36).substring(2, 15)}`
	});

	return `${OAUTH_CONFIG.github.authUrl}?${params.toString()}`;
}

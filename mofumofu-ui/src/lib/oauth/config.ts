// src/lib/oauth/config.ts

import { PUBLIC_GOOGLE_CLIENT_ID, PUBLIC_GITHUB_CLIENT_ID, PUBLIC_APP_URL } from '$env/static/public';

export const OAUTH_CONFIG = {
	google: {
		clientId: PUBLIC_GOOGLE_CLIENT_ID,
		redirectUri: `${PUBLIC_APP_URL}/accounts/oauth/callback/google`,
		scope: 'openid email profile',
		responseType: 'code',
		authUrl: 'https://accounts.google.com/o/oauth2/v2/auth'
	},
	github: {
		clientId: PUBLIC_GITHUB_CLIENT_ID,
		redirectUri: `${PUBLIC_APP_URL}/accounts/oauth/callback/github`,
		scope: 'user:email',
		authUrl: 'https://github.com/login/oauth/authorize'
	}
};

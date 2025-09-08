import { error, redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ url, cookies }) => {
	const state = url.searchParams.get('state');
	const code = url.searchParams.get('code');
	const errorParam = url.searchParams.get('error');

	// If there's an OAuth error, let the client handle it
	if (errorParam) {
		return {
			error: `OAuth error: ${errorParam}`
		};
	}

	// Code is required for OAuth flow
	if (!code) {
		return {
			error: 'Authorization code not found'
		};
	}

	// Check if this is a link request (state starts with 'link_')
	const isLinkRequest = state?.startsWith('link_') ?? false;

	// For link requests, we need different state verification logic
	if (isLinkRequest) {
		// For now, allow link requests to pass through
		// TODO: Implement proper state verification for link requests
		return {
			code,
			handle: null,
			isLinkRequest: true
		};
	}

	// Verify state parameter for CSRF protection
	const storedState = cookies.get('oauth_state');
	if (!state || !storedState || state !== storedState) {
		console.error('State verification failed', { state, storedState });
		throw error(400, 'Invalid state parameter - potential CSRF attack');
	}

	// Get handle if this is a signup flow
	const handle = cookies.get('oauth_handle');

	// Clean up cookies after verification
	cookies.delete('oauth_state', { path: '/' });
	if (handle) {
		cookies.delete('oauth_handle', { path: '/' });
	}

	return {
		code,
		handle: handle || null,
		isLinkRequest: false
	};
};
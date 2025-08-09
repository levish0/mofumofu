import { privateApi } from '../private';
import type { GithubAuthRequest, GoogleAuthRequest, RefreshAccessTokenResponse } from './types';

export async function refreshAccessToken(): Promise<RefreshAccessTokenResponse> {
	try {
		const response = await privateApi.post('v0/auth/refresh', {});
		const data = response.json<RefreshAccessTokenResponse>();
		return data;
	} catch (error) {
		console.error('Failed to refresh access token:', error);
		throw error;
	}
}

export async function signOut(): Promise<void> {
	try {
		await privateApi.post('v0/auth/sign_out', {});
	} catch (error) {
		console.error('Failed to sign out:', error);
		throw error;
	}
}

export async function googleAuth(code: string, handle?: string): Promise<RefreshAccessTokenResponse> {
	const payload: GoogleAuthRequest = { code, handle };
	return privateApi.post('v0/auth/google', { json: payload }).json<RefreshAccessTokenResponse>();
}

export async function githubAuth(code: string, handle?: string): Promise<RefreshAccessTokenResponse> {
	const payload: GithubAuthRequest = { code, handle };
	return privateApi.post('v0/auth/github', { json: payload }).json<RefreshAccessTokenResponse>();
}

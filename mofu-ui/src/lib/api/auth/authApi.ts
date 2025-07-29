import { api } from '../api';
import type { RefreshAccessTokenResponse } from './types';

export async function refreshAccessToken(): Promise<RefreshAccessTokenResponse> {
	try {
		const response = await api.post('v0/auth/refresh', {});
		const data = response.json<RefreshAccessTokenResponse>();
		return data;
	} catch (error) {
		console.error('Failed to refresh access token:', error);
		throw error;
	}
}

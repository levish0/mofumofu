// src/lib/api/user/userApi.ts
import { api } from '../api';
import type { UserInfoResponse } from './types';

export async function getMyProfile(): Promise<UserInfoResponse> {
	try {
		const response = await api.get('v0/user/my_profile');
		return await response.json<UserInfoResponse>();
	} catch (error) {
		console.error(error);
		throw error;
	}
}

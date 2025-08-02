// src/lib/api/user/userApi.ts
import { api } from '../api';
import type { UserInfoResponse, GetUserProfileRequest, UpdateProfileRequest } from './types';

export async function getMyProfile(): Promise<UserInfoResponse> {
	try {
		const response = await api.get('v0/user/my_profile');
		return await response.json<UserInfoResponse>();
	} catch (error) {
		console.error(error);
		throw error;
	}
}

export async function getUserProfile(handle: string): Promise<UserInfoResponse> {
	try {
		const payload: GetUserProfileRequest = { handle };
		const response = await api.post('v0/user/profile', { json: payload });
		return await response.json<UserInfoResponse>();
	} catch (error) {
		console.error(error);
		throw error;
	}
}

export async function updateProfile(data: UpdateProfileRequest): Promise<UserInfoResponse> {
	try {
		const response = await api.put('v0/user/profile', { json: data });
		return await response.json<UserInfoResponse>();
	} catch (error) {
		console.error(error);
		throw error;
	}
}

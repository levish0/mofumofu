import { privateApi } from '../private';
import type { CreateFollowRequest, DeleteFollowRequest } from './types';

export async function createFollow(request: CreateFollowRequest) {
	try {
		await privateApi.post('v0/follow', {
			json: request
		});
	} catch (error) {
		console.error('Failed to create follow:', error);
		throw error;
	}
}

export async function deleteFollow(request: DeleteFollowRequest) {
	try {
		await privateApi.post('v0/unfollow', {
			json: request
		});
	} catch (error) {
		console.error('Failed to delete follow:', error);
		throw error;
	}
}

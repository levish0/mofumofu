import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';
import { getUserProfile, getMyProfile } from '$lib/api/user/userApi';
import { authStore } from '$lib/stores/auth.svelte';

export const load: PageServerLoad = async ({ params, cookies }) => {
	const { slug } = params;
	
	if (!slug) {
		throw error(404, 'Profile not found');
	}

	try {
		// 프로필 정보 가져오기
		const profile = await getUserProfile(slug);
		
		// 현재 로그인한 사용자가 있는지 확인
		let currentUser = null;
		let isOwnProfile = false;
		
		if (authStore.isAuthenticated) {
			try {
				currentUser = await getMyProfile();
				isOwnProfile = currentUser.handle === slug;
			} catch (error) {
				// 토큰이 유효하지 않은 경우 무시
				console.warn('Failed to get current user profile:', error);
			}
		}

		return {
			profile,
			currentUser,
			isOwnProfile
		};
	} catch (err) {
		console.error('Failed to load profile:', err);
		throw error(404, 'Profile not found');
	}
};
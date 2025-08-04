import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';
import { getUserProfile } from '$lib/api/user/userApi';

export const load: PageServerLoad = async ({ params }) => {
	if (!params.slug) {
		throw error(404, 'Profile not found');
	}

	try {
		// Get profile data using public API
		const profile = await getUserProfile(params.slug);
		
		return {
			profile,
			slug: params.slug
		};
	} catch (err) {
		console.error('Failed to load profile:', err);
		throw error(404, 'Profile not found');
	}
};
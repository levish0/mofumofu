import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';
import { getUserProfile } from '$lib/api/user/userApi';

export const load: PageServerLoad = async ({ params }) => {
	if (!params.slug) {
		throw error(404, 'Profile not found');
	}

	try {
		const profile = await getUserProfile(params.slug);
		const timestamp = Date.now();

		// Add timestamp to images to prevent caching
		const profileWithTimestamp = {
			...profile,
			profile_image: profile.profile_image ? `${profile.profile_image}?t=${timestamp}` : null,
			banner_image: profile.banner_image ? `${profile.banner_image}?t=${timestamp}` : null
		};

		return {
			profile: profileWithTimestamp,
			slug: params.slug
		};
	} catch (err) {
		console.error('Failed to load profile:', err);
		throw error(404, 'Profile not found');
	}
};

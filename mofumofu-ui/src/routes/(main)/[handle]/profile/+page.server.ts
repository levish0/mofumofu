import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';
import { getUserProfile } from '$lib/api/user/userApi';

export const load: PageServerLoad = async ({ params }) => {
	if (!params.handle) {
		throw error(404, 'Profile not found');
	}

	// Remove @ prefix if present
	const handle = params.handle.startsWith('@') ? params.handle.slice(1) : params.handle;

	try {
		const profile = await getUserProfile(handle);

		return {
			profile: profile,
			handle: params.handle
		};
	} catch (err) {
		console.error('Failed to load profile:', err);
		throw error(404, 'Profile not found');
	}
};

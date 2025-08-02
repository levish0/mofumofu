import * as v from 'valibot';

export const personalInfoSchema = v.object({
	handle: v.pipe(
		v.string(),
		v.minLength(1, 'Handle is required'),
		v.minLength(3, 'Handle must be at least 3 characters'),
		v.maxLength(20, 'Handle cannot exceed 20 characters'),
		v.regex(/^[a-zA-Z0-9_]+$/, 'Handle can only contain letters, numbers, and underscores')
	),
	name: v.pipe(
		v.string(),
		v.minLength(1, 'Display name is required'),
		v.minLength(3, 'Display name must be at least 3 characters'),
		v.maxLength(20, 'Display name cannot exceed 20 characters')
	),
	profileImage: v.nullable(v.string()),
	bannerImage: v.nullable(v.string()),
	profileImageFile: v.nullable(v.any()),
	bannerImageFile: v.nullable(v.any())
});

export type PersonalInfo = v.InferInput<typeof personalInfoSchema>;

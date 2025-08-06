import * as v from 'valibot';

type Messages = {
	validation_handle_required: () => string;
	validation_handle_min_length: () => string;
	validation_handle_max_length: () => string;
	validation_handle_invalid_format: () => string;
	validation_name_required: () => string;
	validation_name_min_length: () => string;
	validation_name_max_length: () => string;
	validation_bio_max_length: () => string;
	validation_location_max_length: () => string;
	validation_website_max_length: () => string;
	validation_website_invalid_url: () => string;
};

export function createPersonalInfoSchema(m: Messages) {
	return v.object({
		handle: v.pipe(
			v.string(),
			v.minLength(1, m.validation_handle_required()),
			v.minLength(3, m.validation_handle_min_length()),
			v.maxLength(20, m.validation_handle_max_length()),
			v.regex(/^[a-zA-Z0-9_]+$/, m.validation_handle_invalid_format())
		),
		name: v.pipe(
			v.string(),
			v.minLength(1, m.validation_name_required()),
			v.minLength(3, m.validation_name_min_length()),
			v.maxLength(20, m.validation_name_max_length())
		),
		bio: v.pipe(
			v.string(),
			v.maxLength(200, m.validation_bio_max_length())
		),
		location: v.pipe(
			v.string(),
			v.maxLength(30, m.validation_location_max_length())
		),
		website: v.union([
			v.literal(''), // Allow empty string
			v.pipe(
				v.string(),
				v.maxLength(50, m.validation_website_max_length()),
				v.url(m.validation_website_invalid_url())
			)
		]),
		profileImage: v.nullable(v.string()),
		bannerImage: v.nullable(v.string()),
		profileImageFile: v.nullable(v.any()),
		bannerImageFile: v.nullable(v.any())
	});
}

// Fallback schema with English messages
export const personalInfoSchema = createPersonalInfoSchema({
	validation_handle_required: () => 'Handle is required',
	validation_handle_min_length: () => 'Handle must be at least 3 characters',
	validation_handle_max_length: () => 'Handle cannot exceed 20 characters',
	validation_handle_invalid_format: () => 'Handle can only contain letters, numbers, and underscores',
	validation_name_required: () => 'Display name is required',
	validation_name_min_length: () => 'Display name must be at least 3 characters',
	validation_name_max_length: () => 'Display name cannot exceed 20 characters',
	validation_bio_max_length: () => 'Bio must be 200 characters or less',
	validation_location_max_length: () => 'Location must be 30 characters or less',
	validation_website_max_length: () => 'Website must be 50 characters or less',
	validation_website_invalid_url: () => 'Please enter a valid URL'
});

export type PersonalInfo = v.InferInput<ReturnType<typeof createPersonalInfoSchema>>;

export interface UserInfoResponse {
	handle: string;
	name: string;
	email: string;
	bio?: string;
	location?: string;
	website?: string;
	profile_image?: string;
	banner_image?: string;
}

export interface HandleCheckResponse {
	is_available: boolean;
}

export interface CreateUserRequest {
	email: string;
	handle: string;
	name: string;
	password: string;
}

export interface GetUserProfileRequest {
	handle: string;
}

export interface UpdateProfileRequest {
	handle?: string | null;
	name?: string | null;
	bio?: string | null;
	location?: string | null;
	website?: string | null;
	password?: string | null;
}

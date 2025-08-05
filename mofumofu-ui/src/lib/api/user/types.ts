export interface UserInfoResponse {
	handle: string;
	name: string;
	email: string;
	bio?: string;
	profile_image?: string;
	banner_image?: string;
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
	password?: string | null;
}

export interface UserInfoResponse {
	handle: string;
	name: string;
	profile_image?: string;
	banner_image?: string;
}

export interface CreateUserRequest {
	email: string;
	handle: string;
	name: string;
	password: string;
}

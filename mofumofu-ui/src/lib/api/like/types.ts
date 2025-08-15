export interface CreateLikeRequest {
	handle: string;
	slug: string;
}

export interface DeleteLikeRequest {
	handle: string;
	slug: string;
}

export interface CheckLikeStatusRequest {
	handle: string;
	slug: string;
}

export interface LikeStatusResponse {
	is_liked: boolean;
}

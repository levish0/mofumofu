export interface CreatePostRequest {
	title: string;
	content: string;
	slug: string;
	summary?: string | null;
}

export interface GetPostByHandleAndSlugRequest {
	handle: string;
	slug: string;
}

export interface PostInfoResponse {
	title: string;
	content: string;
	user_id: string;
	created_at: string;
	like_count: number;
	comment_count: number;
	view_count: number;
	slug: string;
	summary?: string | null;
	published_at?: string | null;
	updated_at?: string | null;
}

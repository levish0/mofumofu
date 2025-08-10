export interface CreatePostRequest {
	title: string;
	content: string;
	slug: string;
	summary?: string | null;
	hashtags?: string[] | null;
}

export interface GetPostByHandleAndSlugRequest {
	handle: string;
	slug: string;
}

export interface PostAuthor {
	handle: string;
	name: string;
	profile_image?: string | null;
}

export interface PostInfoResponse {
	title: string;
	content: string;
	author: PostAuthor;
	created_at: string;
	like_count: number;
	comment_count: number;
	view_count: number;
	slug: string;
	summary?: string | null;
	published_at?: string | null;
	updated_at?: string | null;
	tags: string[];
}

export type PostSortOrder = 'latest' | 'popular' | 'oldest';

export interface GetPostsRequest {
	page?: number;
	page_size?: number;
	sort?: PostSortOrder;
}

export interface GetPostsAroundPageRequest {
	target_page: number;
	page_size?: number;
	pages_around?: number;
	sort?: PostSortOrder;
}

export interface SearchPostsRequest {
	query?: string | null;
	hashtags?: string[] | null;
	date_from?: string | null;
	date_to?: string | null;
	min_likes?: number | null;
	user_handle?: string | null;
	sort?: PostSortOrder | null;
	target_page?: number | null;
	page_size?: number | null;
	pages_around?: number | null;
}

export interface PostListItem {
	id: string;
	title: string;
	summary?: string | null;
	thumbnail_image?: string | null;
	user_handle: string;
	user_name: string;
	user_avatar?: string | null;
	created_at: string;
	like_count: number;
	comment_count: number;
	view_count: number;
	slug: string;
	hashtags: string[];
}

export interface GetPostsResponse {
	posts: PostListItem[];
	current_page: number;
	page_size: number;
	has_more: boolean;
	total_count?: number | null;
}

export interface ThumbnailUploadRequest {
	slug: string;
	file: File;
}

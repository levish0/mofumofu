import { privateApi } from '../private';
import { publicApi } from '../public';
import type {
	CreatePostRequest,
	GetPostByHandleAndSlugRequest,
	PostInfoResponse,
	GetPostsRequest,
	GetPostsResponse,
	ThumbnailUploadRequest
} from './types';

export async function createPost(postData: CreatePostRequest): Promise<void> {
	try {
		await privateApi.post('v0/post', { json: postData });
	} catch (error) {
		console.error('Failed to create post:', error);
		throw error;
	}
}

export async function getPostByHandleAndSlug(request: GetPostByHandleAndSlugRequest): Promise<PostInfoResponse> {
	try {
		const response = await publicApi.post('v0/post/get', { json: request });
		return response.json<PostInfoResponse>();
	} catch (error) {
		console.error('Failed to get post:', error);
		throw error;
	}
}

export async function getPosts(request: GetPostsRequest = {}): Promise<GetPostsResponse> {
	try {
		const response = await publicApi.post('v0/posts', { json: request });
		return response.json<GetPostsResponse>();
	} catch (error) {
		console.error('Failed to get posts:', error);
		throw error;
	}
}

export async function uploadThumbnail(request: ThumbnailUploadRequest): Promise<void> {
	try {
		const formData = new FormData();
		formData.append('slug', request.slug);
		formData.append('file', request.file);

		await privateApi.post('v0/post/thumbnail', {
			body: formData,
			headers: {
				'Content-Type': undefined
			}
		});
	} catch (error) {
		console.error('Failed to upload thumbnail:', error);
		throw error;
	}
}

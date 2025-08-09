import { privateApi } from '../private';
import { publicApi } from '../public';
import type { 
	CreatePostRequest, 
	GetPostByHandleAndSlugRequest, 
	PostInfoResponse, 
	GetPostsRequest, 
	GetPostsResponse 
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

import type { UserInfoResponse } from '../user/types';

export interface CreateFollowRequest {
  folllowee_handle: string;
}

export interface DeleteFollowRequest {
  followee_handle: string;
}

export interface FollowListResponse {
  has_more: boolean;
  page: number;
  per_page: number;
  total_count: number;
  users: UserInfoResponse[];
}

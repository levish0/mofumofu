import { API_URL } from './config';
import ky from 'ky';
import type { ErrorResponse } from './error/types';
import {
  UserInvalidPassword,
  UserNotFound,
  UserUnauthorized,
  UserTokenExpired,
  UserInvalidToken,
  FollowCannotFollowSelf,
  FollowAlreadyFollowing,
  FollowNotExist,
  BadRequestError,
  SysHashingError,
  SysNotFound,
  SysTransactionError,
  SysDatabaseError,
  SysTokenCreationError,
  ErrorCodes,
  ApiError
} from './error/common_error';
import { ErrorClassMap } from './error/error_class_map';
import { authStore } from '$lib/stores/auth.context.svelte';

function getToken(): string | null {
  return authStore.token || null;
}

export const api = ky.create({
  prefixUrl: API_URL,
  headers: {
    'Content-Type': 'application/json',
    Accept: 'application/json'
  },
  credentials: 'include',
  timeout: 10 * 1000,
  retry: 2,
  hooks: {
    beforeRequest: [
      (request) => {
        const token = getToken();
        if (token) {
          request.headers.set('Authorization', `Bearer ${token}`);
        }
      }
    ],
    afterResponse: [
      async (request, options, response) => {
        if (!response.ok) {
          let errorBody: ErrorResponse | null = null;
          try {
            errorBody = await response.json();
          } catch (error) {
            console.error('Failed to parse error response:', error);
          }
          if (errorBody?.code) {
            const ErrorClass = ErrorClassMap[errorBody.code];
            if (ErrorClass) {
              throw new ErrorClass(errorBody.code, errorBody.status, errorBody);
            } else {
              // 알 수 없는 에러
              throw new ApiError(errorBody.code, errorBody.status, errorBody);
            }
          }
          throw new ApiError('unknown_error', response.status, null);
        }
      }
    ]
  }
});

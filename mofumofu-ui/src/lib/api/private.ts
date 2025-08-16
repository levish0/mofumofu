// src/lib/api/api.ts

import { API_URL } from './config';
import ky from 'ky';
import type { ErrorResponse } from './error/types';
import { ErrorClassMap } from './error/error_class_map';
import { authStore } from '$lib/stores/auth.svelte';
import { ApiError, ErrorCodes } from './error/common_error';
import { refreshAccessToken, signOut } from './auth/authApi';

export const privateApi = ky.create({
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
				const token = authStore.token;
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

					if (errorBody?.code === ErrorCodes.UserNoRefreshToken) {
						authStore.clearToken();
						throw createApiError(errorBody);
					}

					const tokenRefreshNeededCodes = [ErrorCodes.UserTokenExpired, ErrorCodes.UserUnauthorized];

					if (errorBody?.code && tokenRefreshNeededCodes.includes(errorBody.code)) {
						try {
							console.log('Attempting to refresh access token...');
							const refreshResponse = await refreshAccessToken();
							authStore.setToken(refreshResponse.access_token);

							const originalRequest = request.clone();
							originalRequest.headers.set('Authorization', `Bearer ${refreshResponse.access_token}`);

							return ky(originalRequest);
						} catch (refreshError) {
							console.error('Failed to refresh access token:', refreshError);
							// refresh token도 만료된 경우 완전한 로그아웃 처리
							try {
								await signOut();
							} catch (logoutError) {
								console.error('Logout API failed:', logoutError);
							} finally {
								authStore.clearToken();
							}
							throw createApiError(errorBody);
						}
					}

					if (errorBody?.code) {
						throw createApiError(errorBody);
					}
					throw new ApiError('unknown_error', response.status, null);
				}
			}
		]
	}
});

function createApiError(errorBody: ErrorResponse): ApiError {
	const ErrorClass = ErrorClassMap[errorBody.code];
	if (ErrorClass) {
		return new ErrorClass(errorBody.code, errorBody.status, errorBody);
	} else {
		return new ApiError(errorBody.code, errorBody.status, errorBody);
	}
}

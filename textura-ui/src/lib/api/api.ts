// src/lib/api/api.ts
import { API_URL } from './config';
import ky from 'ky';
import type { ErrorResponse } from './error/types';
import { ErrorClassMap } from './error/error_class_map';
import { authStore } from '$lib/stores/auth.svelte';
import { ApiError } from './error/common_error';

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

          if (errorBody?.code) {
            const ErrorClass = ErrorClassMap[errorBody.code];
            if (ErrorClass) {
              throw new ErrorClass(errorBody.code, errorBody.status, errorBody);
            } else {
              throw new ApiError(errorBody.code, errorBody.status, errorBody);
            }
          }
          throw new ApiError('unknown_error', response.status, null);
        }
      }
    ]
  }
});

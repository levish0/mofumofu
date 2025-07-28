// src/lib/api/error/common_error.ts

import type { ErrorResponse } from './types';

export const ErrorCodes = {
  // user module errors
  UserInvalidPassword: 'user:invalid_password',
  UserNotFound: 'user:not_found',
  UserUnauthorized: 'user:unauthorized',
  UserTokenExpired: 'user:token_expired',
  UserInvalidToken: 'user:invalid_token',

  // follow module errors
  FollowCannotFollowSelf: 'follow:cannot_follow_self',
  FollowAlreadyFollowing: 'follow:already_following',
  FollowNotExist: 'follow:not_exist',

  // general module errors
  BadRequest: 'general:bad_request',
  ValidationError: 'general:validation_error',

  // system module errors
  SysHashingError: 'system:hashing_error',
  SysNotFound: 'system:not_found',
  SysTransactionError: 'system:transaction_error',
  SysDatabaseError: 'system:database_error',
  SysTokenCreationError: 'system:token_creation_error'
};

export class ApiError extends Error {
  status: number;
  body: ErrorResponse | null;

  constructor(code: string, status: number, body: ErrorResponse | null) {
    super(code);
    this.status = status;
    this.body = body;
  }
}

export class UserInvalidPassword extends ApiError {}
export class UserNotFound extends ApiError {}
export class UserUnauthorized extends ApiError {}
export class UserTokenExpired extends ApiError {}
export class UserInvalidToken extends ApiError {}

export class FollowCannotFollowSelf extends ApiError {}
export class FollowAlreadyFollowing extends ApiError {}
export class FollowNotExist extends ApiError {}

export class BadRequestError extends ApiError {}
export class ValidationError extends ApiError {}

export class SysHashingError extends ApiError {}
export class SysNotFound extends ApiError {}
export class SysTransactionError extends ApiError {}
export class SysDatabaseError extends ApiError {}
export class SysTokenCreationError extends ApiError {}

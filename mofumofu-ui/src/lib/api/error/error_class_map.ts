// src/lib/api/error/error_class_map.ts

import type { ErrorResponse } from './types';
import {
	ApiError,
	ErrorCodes,
	UserInvalidPassword,
	UserNotFound,
	UserUnauthorized,
	UserTokenExpired,
	UserNoRefreshToken,
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
	ValidationError
} from './common_error';

export const ErrorClassMap: Record<string, new (code: string, status: number, body: ErrorResponse | null) => ApiError> =
	{
		[ErrorCodes.UserInvalidPassword]: UserInvalidPassword,
		[ErrorCodes.UserNotFound]: UserNotFound,
		[ErrorCodes.UserUnauthorized]: UserUnauthorized,
		[ErrorCodes.UserTokenExpired]: UserTokenExpired,
		[ErrorCodes.UserNoRefreshToken]: UserNoRefreshToken,
		[ErrorCodes.UserInvalidToken]: UserInvalidToken,

		[ErrorCodes.FollowCannotFollowSelf]: FollowCannotFollowSelf,
		[ErrorCodes.FollowAlreadyFollowing]: FollowAlreadyFollowing,
		[ErrorCodes.FollowNotExist]: FollowNotExist,

		[ErrorCodes.BadRequest]: BadRequestError,
		[ErrorCodes.ValidationError]: ValidationError,

		[ErrorCodes.SysHashingError]: SysHashingError,
		[ErrorCodes.SysNotFound]: SysNotFound,
		[ErrorCodes.SysTransactionError]: SysTransactionError,
		[ErrorCodes.SysDatabaseError]: SysDatabaseError,
		[ErrorCodes.SysTokenCreationError]: SysTokenCreationError
	};

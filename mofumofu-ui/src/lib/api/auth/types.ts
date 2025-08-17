export interface RefreshAccessTokenResponse {
	access_token: string;
}

export interface GoogleAuthRequest {
	code: string;
	handle?: string;
}

export interface GithubAuthRequest {
	code: string;
	handle?: string;
}

export interface SignupRequest {
	name: string;
	handle: string;
	password: string;
	email: string;
}

export interface VerifyEmailRequest {
	token: string;
}

export interface SigninRequest {
	handle: string;
	password: string;
}

export interface ForgotPasswordRequest {
	email: string;
}

export interface ResetPasswordRequest {
	token: string;
	new_password: string;
}

export interface ResendVerificationRequest {
	email: string;
}

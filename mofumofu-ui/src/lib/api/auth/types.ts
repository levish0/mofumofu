export interface RefreshAccessTokenResponse {
	access_token: string;
}

export interface GoogleLoginRequest {
	code: string;
	handle: string;
}

export interface GithubLoginRequest {
	code: string;
	handle: string;
}

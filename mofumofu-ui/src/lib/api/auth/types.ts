export interface RefreshAccessTokenResponse {
	access_token: string;
}

export interface GoogleLoginRequest {
	code: string;
}

export interface GithubLoginRequest {
	code: string;
}

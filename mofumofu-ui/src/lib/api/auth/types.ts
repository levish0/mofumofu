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

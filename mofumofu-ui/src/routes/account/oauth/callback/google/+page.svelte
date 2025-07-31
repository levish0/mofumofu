<!-- src/routes/accounts/oauth/callback/google/+page.svelte -->
<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { googleSignIn } from '$lib/api/auth/authApi';
	import { goto } from '$app/navigation';
	import { authStore } from '$lib/stores/auth.svelte';
	import { ApiError } from '$lib/api/error/common_error';

	let loading = $state(true);
	let error = $state<string | null>(null);

	onMount(async () => {
		try {
			const code = page.url.searchParams.get('code');
			const errorParam = page.url.searchParams.get('error');

			if (errorParam) {
				throw new Error(`OAuth error: ${errorParam}`);
			}

			if (!code) {
				throw new Error('Authorization code not found');
			}

			// Google OAuth 로그인 처리
			const response = await googleSignIn(code);

			// 토큰을 스토어에 저장
			authStore.setToken(response.access_token);

			// 성공 시 메인 페이지로 리다이렉트
			await goto('/', { replaceState: true });
		} catch (err) {
			console.error('Google OAuth error:', err);

			if (err instanceof ApiError) {
				error = `Login failed: ${err.message}`;
			} else if (err instanceof Error) {
				error = err.message;
			} else {
				error = 'An unexpected error occurred during login';
			}
		} finally {
			loading = false;
		}
	});
</script>

<div class="flex min-h-screen items-center justify-center bg-gray-50">
	<div class="w-full max-w-md space-y-8 p-8">
		<div class="text-center">
			{#if loading}
				<div class="space-y-4">
					<div class="mx-auto h-12 w-12 animate-spin rounded-full border-b-2 border-blue-600"></div>
					<h2 class="text-xl font-semibold text-gray-900">Processing Google login...</h2>
					<p class="text-gray-600">Please wait while we sign you in</p>
				</div>
			{:else if error}
				<div class="space-y-4">
					<div class="text-red-600">
						<svg class="mx-auto h-12 w-12" fill="none" viewBox="0 0 24 24" stroke="currentColor">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.962-.833-2.732 0L3.082 16.5c-.77.833.192 2.5 1.732 2.5z"
							/>
						</svg>
					</div>
					<h2 class="text-xl font-semibold text-gray-900">Login Failed</h2>
					<p class="text-red-600">{error}</p>
					<button
						onclick={() => goto('/account/signin')}
						class="mt-4 rounded-md bg-blue-600 px-4 py-2 text-white hover:bg-blue-700 focus:ring-2 focus:ring-blue-500 focus:outline-none"
					>
						Back to Login
					</button>
				</div>
			{/if}
		</div>
	</div>
</div>

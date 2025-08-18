<script lang="ts">
	import { Input } from '../../../ui/input';
	import { Button } from '../../../ui/button';
	import { setPassword, getOAuthConnections } from '$lib/api/auth/authApi';
	import type { OAuthConnectionsResponse } from '$lib/api/auth/types';
	import { toast } from 'svelte-sonner';
	import { onMount } from 'svelte';
	import * as v from 'valibot';

	let password = $state('');
	let confirmPassword = $state('');
	let isSettingPassword = $state(false);
	let isOAuthOnly = $state(false);
	let showPasswordFields = $state(false);

	const passwordSchema = v.pipe(
		v.string(),
		v.minLength(8, '비밀번호는 최소 8자 이상이어야 합니다.'),
		v.regex(/^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)/, '비밀번호는 대문자, 소문자, 숫자를 포함해야 합니다.')
	);

	const passwordError = $derived(() => {
		if (!password) return undefined;
		const result = v.safeParse(passwordSchema, password);
		return result.success ? undefined : result.issues?.[0]?.message;
	});

	const confirmPasswordError = $derived(() => {
		if (!confirmPassword) return undefined;
		return password !== confirmPassword ? '비밀번호가 일치하지 않습니다.' : undefined;
	});

	const isFormValid = $derived(
		password && 
		confirmPassword && 
		!passwordError() && 
		!confirmPasswordError()
	);

	onMount(async () => {
		await checkOAuthStatus();
	});

	async function checkOAuthStatus() {
		try {
			const response = await getOAuthConnections();
			isOAuthOnly = response.is_oauth_only;
		} catch (error) {
			console.error('Failed to check OAuth status:', error);
		}
	}

	async function handleSetPassword() {
		if (!isFormValid) return;

		isSettingPassword = true;

		try {
			await setPassword(password);
			await checkOAuthStatus();
			toast.success('비밀번호가 성공적으로 설정되었습니다.');
			
			// 폼 초기화
			password = '';
			confirmPassword = '';
			showPasswordFields = false;
		} catch (error) {
			console.error('Failed to set password:', error);
			toast.error('비밀번호 설정에 실패했습니다. 다시 시도해주세요.');
		} finally {
			isSettingPassword = false;
		}
	}

	function handleShowPasswordFields() {
		showPasswordFields = true;
	}

	function handleCancel() {
		showPasswordFields = false;
		password = '';
		confirmPassword = '';
	}
</script>

{#if isOAuthOnly}
	<div class="space-y-4">
		<h2 class="text-2xl font-semibold">비밀번호 설정</h2>
		<div class="space-y-3">
			{#if !showPasswordFields}
				<div class="rounded-lg border border-orange-400/20 bg-orange-400/10 p-4">
					<div class="flex items-start space-x-3">
						<div class="flex-shrink-0">
							<svg class="h-5 w-5 text-orange-400" fill="currentColor" viewBox="0 0 20 20">
								<path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd"/>
							</svg>
						</div>
						<div class="flex-1">
							<h3 class="text-sm font-medium text-orange-400">소셜 로그인 전용 계정</h3>
							<p class="mt-1 text-sm text-mofu-light-400 dark:text-mofu-dark-400">
								현재 소셜 로그인만으로 가입된 계정입니다. 비밀번호를 설정하면 이메일과 비밀번호로도 로그인할 수 있습니다.
							</p>
							<div class="mt-3">
								<Button class="h-8 px-3 text-sm" onclick={handleShowPasswordFields}>
									비밀번호 설정하기
								</Button>
							</div>
						</div>
					</div>
				</div>
			{:else}
				<div class="space-y-4">
					<div class="space-y-2">
						<label for="password" class="text-sm font-medium text-mofu-light-200 dark:text-mofu-dark-200">
							새 비밀번호
						</label>
						<Input
							id="password"
							type="password"
							placeholder="새 비밀번호를 입력하세요"
							class="dark:bg-mofu-dark-800 bg-mofu-light-800 text-mofu-light-200 dark:text-mofu-dark-200 placeholder:text-mofu-light-300 dark:placeholder:text-mofu-dark-300 {passwordError() ? 'border-rose-500' : ''}"
							bind:value={password}
						/>
						{#if passwordError()}
							<p class="text-xs text-rose-400">{passwordError()}</p>
						{/if}
					</div>

					<div class="space-y-2">
						<label for="confirmPassword" class="text-sm font-medium text-mofu-light-200 dark:text-mofu-dark-200">
							비밀번호 확인
						</label>
						<Input
							id="confirmPassword"
							type="password"
							placeholder="비밀번호를 다시 입력하세요"
							class="dark:bg-mofu-dark-800 bg-mofu-light-800 text-mofu-light-200 dark:text-mofu-dark-200 placeholder:text-mofu-light-300 dark:placeholder:text-mofu-dark-300 {confirmPasswordError() ? 'border-rose-500' : ''}"
							bind:value={confirmPassword}
						/>
						{#if confirmPasswordError()}
							<p class="text-xs text-rose-400">{confirmPasswordError()}</p>
						{/if}
					</div>

					<div class="flex space-x-3">
						<Button
							onclick={handleSetPassword}
							disabled={!isFormValid || isSettingPassword}
						>
							{#if isSettingPassword}
								<div class="h-4 w-4 animate-spin rounded-full border-2 border-current border-t-transparent"></div>
							{:else}
								비밀번호 설정
							{/if}
						</Button>
						<Button variant="outline" onclick={handleCancel}>
							취소
						</Button>
					</div>
				</div>
			{/if}
		</div>
	</div>
{/if}
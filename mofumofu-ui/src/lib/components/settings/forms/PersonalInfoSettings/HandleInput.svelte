<script lang="ts">
	import { Input } from '../../../ui/input';
	import * as v from 'valibot';
	import { createPersonalInfoSchema } from '$lib/schemas/personal-info';
	import * as m from '../../../../../paraglide/messages';
	import { personalSettingsStore } from '$lib/stores/settings.svelte';

	interface Props {
		handle: string | null;
		onUpdate: (handle: string) => void;
		onValidationChange: (error?: string) => void;
		isVerified: boolean;
	}

	let { handle, onUpdate, onValidationChange, isVerified }: Props = $props();

	let localError = $state<string | undefined>();

	// 스토어에서 핸들 검증 상태 가져오기
	const handleVerificationState = $derived(personalSettingsStore.handleVerificationStatus);
	const needsVerification = $derived(personalSettingsStore.handleNeedsVerification);

	function validateHandle(value: string): string | undefined {
		const schema = createPersonalInfoSchema();
		const result = v.safeParse(schema.entries.handle, value.trim());
		return result.success ? undefined : result.issues?.[0]?.message;
	}

	async function checkHandleAvailability() {
		if (!handle?.trim() || validateHandle(handle || '')) {
			return;
		}

		await personalSettingsStore.checkHandleAvailability();
	}

	function handleInput(e: Event) {
		if (!isVerified) {
			alert('이메일 인증이 필요합니다. 이메일을 확인해주세요.');
			return;
		}
		
		const value = (e.target as HTMLInputElement).value;
		onUpdate(value);

		const error = validateHandle(value);
		localError = error;
		onValidationChange(error);
	}

	const characterCount = $derived((handle || '').length);
	const isOverLimit = $derived(characterCount > 20);
</script>

<div class="space-y-4">
	<h2 class="text-2xl font-semibold">{m.settings_handle()}</h2>
	<div class="space-y-2">
		<div class="relative flex">
			<span class="dark:bg-mofu-dark-800/50 text-mofu-dark-200 inline-flex items-center rounded-l-md px-3 text-sm"
				>@</span
			>
			<div class="relative flex-1">
				<Input
					id="handle"
					placeholder={m.settings_handle_placeholder()}
					class="dark:bg-mofu-dark-800 text-mofu-dark-200 placeholder:text-mofu-dark-300 rounded-l-none rounded-r-none border-r-0 pr-12 {localError
						? 'border-red-500'
						: ''} {!isVerified ? 'opacity-50 cursor-not-allowed' : ''}"
					value={handle || ''}
					oninput={handleInput}
					disabled={!isVerified}
				/>
				<div
					class="absolute top-1/2 right-3 -translate-y-1/2 text-xs {isOverLimit
						? 'text-red-400'
						: 'text-mofu-dark-400'}"
				>
					{characterCount}/20
				</div>
			</div>
			<button
				onclick={checkHandleAvailability}
				disabled={handleVerificationState === 'checking' || !handle?.trim() || !!validateHandle(handle || '')}
				class="dark:bg-mofu-dark-800/50 text-mofu-dark-200 disabled:dark:bg-mofu-dark-800/50 hover:bg-mofu-dark-700 inline-flex min-w-20 items-center justify-center rounded-r-md px-3 py-2 text-sm font-medium disabled:cursor-not-allowed disabled:opacity-50"
			>
				{#if handleVerificationState === 'checking'}
					<svg class="h-4 w-4 animate-spin" fill="none" viewBox="0 0 24 24">
						<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
						<path
							class="opacity-75"
							fill="currentColor"
							d="m4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
						></path>
					</svg>
				{:else}
					{m.settings_handle_check()}
				{/if}
			</button>
		</div>
		{#if !isVerified}
			<p class="text-xs text-gray-500">이메일 인증이 필요합니다. 핸들 변경을 위해 이메일을 인증해주세요.</p>
		{:else if localError}
			<p class="text-xs text-rose-400">{localError}</p>
		{:else if handleVerificationState === 'verified' && !needsVerification}
			<p class="text-xs text-green-400">{m.settings_handle_available()}</p>
		{:else if handleVerificationState === 'unavailable'}
			<p class="text-xs text-rose-400">{m.settings_handle_taken()}</p>
		{:else if needsVerification}
			<p class="text-xs text-orange-400">{m.settings_handle_verification_required()}</p>
		{:else}
			<p class="text-mofu-dark-400 text-xs">{m.settings_handle_description()}</p>
		{/if}
	</div>
</div>

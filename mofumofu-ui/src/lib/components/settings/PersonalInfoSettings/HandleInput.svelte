<script lang="ts">
	import { Input } from '../../ui/input';
	import * as v from 'valibot';
	import { personalInfoSchema } from '$lib/schemas/personal-info';
	import * as m from '../../../../paraglide/messages';

	interface Props {
		handle: string | null;
		onUpdate: (handle: string) => void;
		onValidationChange: (error?: string) => void;
	}

	let { handle, onUpdate, onValidationChange }: Props = $props();

	let localError = $state<string | undefined>();
	let isChecking = $state(false);
	let handleAvailable = $state<boolean | null>(null);

	function validateHandle(value: string): string | undefined {
		const result = v.safeParse(personalInfoSchema.entries.handle, value.trim());
		return result.success ? undefined : result.issues?.[0]?.message;
	}

	async function checkHandleAvailability() {
		if (!handle?.trim() || validateHandle(handle || '')) {
			return;
		}

		isChecking = true;
		handleAvailable = null;

		await new Promise((resolve) => setTimeout(resolve, 1000));

		const unavailableHandles = ['admin', 'test', 'user', 'mofu'];
		handleAvailable = !unavailableHandles.includes((handle || '').toLowerCase());

		isChecking = false;
	}

	function handleInput(e: Event) {
		const value = (e.target as HTMLInputElement).value;
		onUpdate(value);

		const error = validateHandle(value);
		localError = error;
		onValidationChange(error);

		handleAvailable = null;
	}
</script>

<div class="space-y-4">
	<h2 class="text-2xl font-semibold">{m.settings_handle()}</h2>
	<div class="space-y-2">
		<div class="flex">
			<span class="dark:bg-mofu-dark-800/50 text-mofu-dark-200 inline-flex items-center rounded-l-md px-3 text-sm"
				>@</span
			>
			<Input
				id="handle"
				placeholder={m.settings_handle_placeholder()}
				class="dark:bg-mofu-dark-800 text-mofu-dark-200 placeholder:text-mofu-dark-300 rounded-l-none rounded-r-none border-r-0 {localError
					? 'border-red-500'
					: ''}"
				value={handle || ''}
				oninput={handleInput}
			/>
			<button
				onclick={checkHandleAvailability}
				disabled={isChecking || !handle?.trim() || !!validateHandle(handle || '')}
				class="dark:bg-mofu-dark-800/50 text-mofu-dark-200 disabled:dark:bg-mofu-dark-800/50 hover:bg-mofu-dark-700 inline-flex min-w-20 items-center justify-center rounded-r-md px-3 py-2 text-sm font-medium disabled:cursor-not-allowed disabled:opacity-50"
			>
				{#if isChecking}
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
		{#if localError}
			<p class="text-xs text-rose-400">{localError}</p>
		{:else if handleAvailable === true}
			<p class="text-xs text-green-400">{m.settings_handle_available()}</p>
		{:else if handleAvailable === false}
			<p class="text-xs text-rose-400">{m.settings_handle_taken()}</p>
		{:else}
			<p class="text-xs text-gray-500">{m.settings_handle_description()}</p>
		{/if}
	</div>
</div>

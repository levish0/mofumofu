<script lang="ts">
	import { Input } from '../../ui/input';
	import * as v from 'valibot';
	import { createPersonalInfoSchema } from '$lib/schemas/personal-info';
	import * as m from '../../../../paraglide/messages';

	interface Props {
		website: string | null;
		onUpdate: (website: string) => void;
		onValidationChange: (error?: string) => void;
	}

	let { website, onUpdate, onValidationChange }: Props = $props();

	let localError = $state<string | undefined>();

	function validateWebsite(value: string): string | undefined {
		const schema = createPersonalInfoSchema(m);
		const result = v.safeParse(schema.entries.website, value.trim());
		return result.success ? undefined : result.issues?.[0]?.message;
	}

	function handleInput(e: Event) {
		const value = (e.target as HTMLInputElement).value;
		onUpdate(value);

		const error = validateWebsite(value);
		localError = error;
		onValidationChange(error);
	}

	const characterCount = $derived((website || '').length);
	const isOverLimit = $derived(characterCount > 50);
</script>

<div class="space-y-4">
	<h2 class="text-2xl font-semibold">{m.settings_website()}</h2>
	<div class="space-y-2">
		<div class="relative">
			<Input
				id="website"
				placeholder={m.settings_website_placeholder()}
				class="dark:bg-mofu-dark-800 text-mofu-dark-200 placeholder:text-mofu-dark-300 pr-12 {localError
					? 'border-red-500'
					: ''}"
				value={website || ''}
				oninput={handleInput}
			/>
			<div
				class="absolute top-1/2 right-3 -translate-y-1/2 text-xs {isOverLimit ? 'text-red-400' : 'text-mofu-dark-400'}"
			>
				{characterCount}/50
			</div>
		</div>
		{#if localError}
			<p class="text-xs text-rose-400">{localError}</p>
		{:else}
			<p class="text-mofu-dark-400 text-xs">{m.settings_website_description()}</p>
		{/if}
	</div>
</div>

<script lang="ts">
	import { Textarea } from '../../../ui/textarea';
	import * as v from 'valibot';
	import { createPersonalInfoSchema } from '$lib/schemas/personal-info';
	import * as m from '../../../../../paraglide/messages';

	interface Props {
		bio: string | null;
		onUpdate: (bio: string) => void;
		onValidationChange: (error?: string) => void;
	}

	let { bio, onUpdate, onValidationChange }: Props = $props();

	let localError = $state<string | undefined>();

	function validateBio(value: string): string | undefined {
		const schema = createPersonalInfoSchema();
		const result = v.safeParse(schema.entries.bio, value.trim());
		return result.success ? undefined : result.issues?.[0]?.message;
	}

	function handleInput(e: Event) {
		const value = (e.target as HTMLTextAreaElement).value;
		onUpdate(value);

		const error = validateBio(value);
		localError = error;
		onValidationChange(error);
	}

	const characterCount = $derived((bio || '').length);
	const isOverLimit = $derived(characterCount > 200);
</script>

<div class="space-y-4">
	<h2 class="text-2xl font-semibold">{m.settings_bio()}</h2>
	<div class="space-y-2">
		<div class="relative">
			<Textarea
				id="bio"
				placeholder={m.settings_bio_placeholder()}
				class="dark:bg-mofu-dark-800 text-mofu-dark-200 placeholder:text-mofu-dark-300 min-h-[100px]  {localError
					? 'border-red-500'
					: ''}"
				value={bio || ''}
				oninput={handleInput}
			/>
			<div class="absolute right-2 bottom-2 text-xs {isOverLimit ? 'text-red-400' : 'text-mofu-dark-400'}">
				{characterCount}/200
			</div>
		</div>
		{#if localError}
			<p class="text-xs text-rose-400">{localError}</p>
		{:else}
			<p class="text-mofu-dark-400 text-xs">{m.settings_bio_description()}</p>
		{/if}
	</div>
</div>

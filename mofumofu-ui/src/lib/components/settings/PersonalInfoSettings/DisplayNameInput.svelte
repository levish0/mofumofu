<script lang="ts">
	import { Input } from '../../ui/input';
	import * as v from 'valibot';
	import { personalInfoSchema } from '$lib/schemas/personal-info';
	import * as m from '../../../../paraglide/messages';

	interface Props {
		name: string | null;
		onUpdate: (name: string) => void;
		onValidationChange: (error?: string) => void;
	}

	let { name, onUpdate, onValidationChange }: Props = $props();

	let localError = $state<string | undefined>();

	function validateName(value: string): string | undefined {
		const result = v.safeParse(personalInfoSchema.entries.name, value.trim());
		return result.success ? undefined : result.issues?.[0]?.message;
	}

	function handleInput(e: Event) {
		const value = (e.target as HTMLInputElement).value;
		onUpdate(value);

		const error = validateName(value);
		localError = error;
		onValidationChange(error);
	}
</script>

<div class="space-y-4">
	<h2 class="text-2xl font-semibold">{m.settings_display_name()}</h2>
	<div class="space-y-2">
		<Input
			id="name"
			placeholder={m.settings_display_name_placeholder()}
			class="dark:bg-mofu-dark-800 text-mofu-dark-200 placeholder:text-mofu-dark-300 {localError
				? 'border-red-500'
				: ''}"
			value={name || ''}
			oninput={handleInput}
		/>
		{#if localError}
			<p class="text-xs text-rose-400">{localError}</p>
		{:else}
			<p class="text-mofu-dark-400 text-xs">{m.settings_display_name_description()}</p>
		{/if}
	</div>
</div>

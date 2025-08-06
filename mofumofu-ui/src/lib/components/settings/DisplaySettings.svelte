<script lang="ts">
	import * as Select from '$lib/components/ui/select';
	import * as m from '../../../paraglide/messages';
	import { setLocale, getLocale } from '../../../paraglide/runtime';
	import { onMount } from 'svelte';

	let selectedLanguage = $state('en');

	const languages = [
		{ value: 'en', label: m.language_english },
		{ value: 'ja', label: m.language_japanese },
		{ value: 'ko', label: m.language_korean },
		{ value: 'de', label: m.language_german },
		{ value: 'es', label: m.language_spanish },
		{ value: 'es-MX', label: m.language_spanish_mx },
		{ value: 'fr', label: m.language_french },
		{ value: 'ru', label: m.language_russian }
	];

	const triggerContent = $derived(languages.find((l) => l.value === selectedLanguage)?.label() ?? 'Select language');

	onMount(() => {
		selectedLanguage = getLocale();
	});

	function handleLanguageChange(value: string | undefined) {
		if (value && isValidLocale(value)) {
			selectedLanguage = value;
			setLocale(value as 'en' | 'ko' | 'ja' | 'de' | 'es' | 'es-MX' | 'fr' | 'ru');
		}
	}

	function isValidLocale(value: string): value is 'ja' | 'en' | 'ko' | 'de' | 'es' | 'es-MX' | 'fr' | 'ru' {
		return ['ja', 'en', 'ko', 'de', 'es', 'es-MX', 'fr', 'ru'].includes(value);
	}
</script>

<div class="text-mofu-dark-200 min-h-screen">
	<div class="space-y-4">
		<h2 class="text-2xl font-semibold">{m.settings_language()}</h2>
		<div class="space-y-2">
			<Select.Root type="single" bind:value={selectedLanguage} onValueChange={handleLanguageChange}>
				<Select.Trigger class="dark:bg-mofu-dark-800 text-mofu-dark-200 border-mofu-dark-700 w-64">
					{triggerContent}
				</Select.Trigger>
				<Select.Content class="dark:bg-mofu-dark-800 border-mofu-dark-700">
					{#each languages as language}
						<Select.Item value={language.value} class="text-mofu-dark-200 hover:bg-mofu-dark-700">
							{language.label()}
						</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
			<p class="text-mofu-dark-400 text-xs">{m.settings_language_description()}</p>
		</div>
	</div>
</div>

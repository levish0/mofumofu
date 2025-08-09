<script lang="ts">
	import { Textarea } from '$lib/components/ui/textarea';
	import * as v from 'valibot';
	import { createPostSchema } from '$lib/schemas/post';

	interface Props {
		value: string;
		onUpdate: (value: string) => void;
		onValidationChange: (error?: string) => void;
	}

	let { value, onUpdate, onValidationChange }: Props = $props();

	let validationError = $state<string>('');
	const summaryCount = $derived(value.length);

	function validateSummary(summaryValue: string) {
		const schema = createPostSchema();
		const dataToValidate = {
			title: 'temp',
			content: 'temp',
			slug: 'temp',
			summary: summaryValue,
			tags: ''
		};

		const result = v.safeParse(schema, dataToValidate);
		
		if (result.success) {
			validationError = '';
			onValidationChange();
		} else {
			const summaryError = result.issues.find(issue => 
				issue.path?.[0]?.key === 'summary'
			);
			if (summaryError) {
				validationError = summaryError.message;
				onValidationChange(summaryError.message);
			}
		}
	}

	function handleInput() {
		onUpdate(value);
		validateSummary(value);
	}
</script>

<div>
	<label for="summary" class="text-mofu-dark-100 mb-2 block text-sm font-medium">요약 (선택사항)</label>
	<div class="relative">
		<Textarea
			id="summary"
			bind:value
			oninput={handleInput}
			placeholder="포스트 요약을 입력하세요"
			class="bg-mofu-dark-700 border-mofu-dark-600 placeholder:text-mofu-dark-400 min-h-[80px] text-white {validationError ? 'border-red-500' : ''}"
		/>
		<div class="absolute right-2 bottom-2 text-xs {validationError ? 'text-red-400' : 'text-mofu-dark-400'}">
			{summaryCount}/500
		</div>
	</div>
	{#if validationError}
		<p class="mt-1 text-xs text-rose-400">{validationError}</p>
	{/if}
</div>
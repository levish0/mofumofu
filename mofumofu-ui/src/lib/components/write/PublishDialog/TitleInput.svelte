<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import * as v from 'valibot';
	import { createPostSchema } from '$lib/schemas/post';

	interface Props {
		value: string;
		onUpdate: (value: string) => void;
		onValidationChange: (error?: string) => void;
	}

	let { value, onUpdate, onValidationChange }: Props = $props();

	let validationError = $state<string>('');
	const titleCount = $derived(value.length);

	function validateTitle(titleValue: string) {
		const schema = createPostSchema();
		const dataToValidate = {
			title: titleValue,
			content: 'temp',
			slug: 'temp',
			summary: '',
			tags: ''
		};

		const result = v.safeParse(schema, dataToValidate);
		
		if (result.success) {
			validationError = '';
			onValidationChange();
		} else {
			const titleError = result.issues.find(issue => 
				issue.path?.[0]?.key === 'title'
			);
			if (titleError) {
				validationError = titleError.message;
				onValidationChange(titleError.message);
			}
		}
	}

	function handleInput(e: Event) {
		const newValue = (e.target as HTMLInputElement).value;
		onUpdate(newValue);
		validateTitle(newValue);
	}
</script>

<div>
	<label for="title" class="text-mofu-dark-100 mb-2 block text-sm font-medium">제목</label>
	<div class="relative">
		<Input
			id="title"
			{value}
			oninput={handleInput}
			placeholder="포스트 제목을 입력하세요"
			class="dark:bg-mofu-dark-700 border-mofu-dark-600 placeholder:text-mofu-dark-400 pr-12 text-white {validationError ? 'border-red-500' : ''}"
		/>
		<div class="absolute top-1/2 right-3 -translate-y-1/2 text-xs {validationError ? 'text-red-400' : 'text-mofu-dark-400'}">
			{titleCount}/80
		</div>
	</div>
	{#if validationError}
		<p class="mt-1 text-xs text-rose-400">{validationError}</p>
	{/if}
</div>
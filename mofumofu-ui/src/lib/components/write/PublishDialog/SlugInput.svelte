<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import * as v from 'valibot';
	import { createPostSchema } from '$lib/schemas/post';
	import { userStore } from '$lib/stores/user.svelte';

	interface Props {
		value: string;
		onUpdate: (value: string) => void;
		onValidationChange: (error?: string) => void;
	}

	let { value, onUpdate, onValidationChange }: Props = $props();

	let validationError = $state<string>('');
	const slugCount = $derived(value.length);

	function generateSlug(text: string): string {
		return text
			.trim()
			// URL에 안전하지 않은 문자들만 제거 (공백, /, ?, #, [, ], @, !, $, &, ', (, ), *, +, ,, ;, =)
			.replace(/[\s\/\?#\[\]@!$&'()*+,;=]+/g, '-')
			// 연속된 하이픈을 하나로
			.replace(/-+/g, '-')
			// 앞뒤 하이픈 제거
			.replace(/^-+|-+$/g, '');
	}

	function validateSlug(slugValue: string) {
		const schema = createPostSchema();
		const dataToValidate = {
			title: 'temp',
			content: 'temp',
			slug: slugValue,
			summary: '',
			tags: ''
		};

		const result = v.safeParse(schema, dataToValidate);
		
		if (result.success) {
			validationError = '';
			onValidationChange();
		} else {
			const slugError = result.issues.find(issue => 
				issue.path?.[0]?.key === 'slug'
			);
			if (slugError) {
				validationError = slugError.message;
				onValidationChange(slugError.message);
			}
		}
	}

	function handleInput(e: Event) {
		const inputValue = (e.target as HTMLInputElement).value;
		const newValue = generateSlug(inputValue);
		onUpdate(newValue);
		validateSlug(newValue);
	}
</script>

<div>
	<label for="slug" class="text-mofu-dark-100 mb-2 block text-sm font-medium">슬러그</label>
	<div class="relative">
		<Input
			id="slug"
			{value}
			oninput={handleInput}
			placeholder="URL에 사용될 슬러그"
			class="dark:bg-mofu-dark-700 border-mofu-dark-600 placeholder:text-mofu-dark-400 pr-12 text-white {validationError ? 'border-red-500' : ''}"
		/>
		<div class="absolute top-1/2 right-3 -translate-y-1/2 text-xs {validationError ? 'text-red-400' : 'text-mofu-dark-400'}">
			{slugCount}/80
		</div>
	</div>
	{#if validationError}
		<p class="mt-1 text-xs text-rose-400">{validationError}</p>
	{/if}
	<p class="text-mofu-dark-400 mt-1 text-xs">
		@{userStore.user?.handle || '사용자핸들'}/{value}
	</p>
</div>
<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Badge } from '$lib/components/ui/badge';
	import * as Dialog from '$lib/components/ui/dialog';
	import { createPost } from '$lib/api/post/postApi';
	import type { CreatePostRequest } from '$lib/api/post/types';
	import * as v from 'valibot';
	import { createPostSchema } from '$lib/schemas/post';
	import { Icon, PaperAirplane } from 'svelte-hero-icons';
	import { userStore } from '$lib/stores/user.svelte';

	interface Props {
		title: string;
		content: string;
		tags: string;
		onPublished?: () => void;
	}

	let { title, content, tags, onPublished }: Props = $props();

	let isOpen = $state(false);
	let isLoading = $state(false);
	let publishData = $state({
		title: '',
		slug: '',
		content: '',
		summary: '',
		tags: ''
	});
	let validationErrors = $state<Record<string, string>>({});

	function generateSlug(text: string): string {
		return text
			.toLowerCase()
			.trim()
			.replace(/[^\w\s-]/g, '')
			.replace(/[\s_-]+/g, '-')
			.replace(/^-+|-+$/g, '');
	}

	function openDialog() {
		publishData = {
			title: title,
			slug: generateSlug(title),
			content: content,
			summary: '',
			tags: tags
		};
		isOpen = true;
	}

	function validateField(field: string, value: string) {
		const schema = createPostSchema();

		// 개별 필드 검증을 위한 임시 객체 생성
		const dataToValidate: any = {
			title: field === 'title' ? value : publishData.title || 'temp',
			content: field === 'content' ? value : publishData.content || 'temp',
			slug: field === 'slug' ? value : publishData.slug || 'temp',
			summary: field === 'summary' ? value : publishData.summary,
			tags: field === 'tags' ? value : publishData.tags
		};

		const result = v.safeParse(schema, dataToValidate);

		if (result.success) {
			validationErrors[field] = '';
		} else {
			// 해당 필드의 에러만 찾아서 설정
			const fieldError = result.issues.find((issue) => issue.path?.[0]?.key === field);
			if (fieldError) {
				validationErrors[field] = fieldError.message;
			}
		}
	}

	async function handlePublish() {
		// Clear previous errors
		validationErrors = {};

		// Validate all fields
		const schema = createPostSchema();
		const dataToValidate = {
			title: publishData.title.trim(),
			content: publishData.content.trim(),
			slug: publishData.slug.trim(),
			summary: publishData.summary.trim(),
			tags: publishData.tags
		};

		const result = v.safeParse(schema, dataToValidate);

		if (!result.success) {
			result.issues.forEach((issue) => {
				const path = issue.path?.[0]?.key as string;
				if (path) {
					validationErrors[path] = issue.message;
				}
			});
			return;
		}

		try {
			isLoading = true;

			const postRequest: CreatePostRequest = {
				title: publishData.title.trim(),
				content: publishData.content.trim(),
				slug: publishData.slug.trim(),
				summary: publishData.summary.trim() || null
			};

			await createPost(postRequest);

			isOpen = false;
			onPublished?.();
		} catch (error) {
			console.error('출간 실패:', error);
			alert('출간에 실패했습니다. 다시 시도해주세요.');
		} finally {
			isLoading = false;
		}
	}

	function handleSlugChange(value: string) {
		publishData.slug = generateSlug(value);
	}

	// Character count derived values (UI 표시용)
	const titleCount = $derived(publishData.title.length);
	const slugCount = $derived(publishData.slug.length);
	const summaryCount = $derived(publishData.summary.length);
	const tagArray = $derived([
		...new Set(
			publishData.tags
				.split(/[,\n]/)
				.map((tag) => tag.trim())
				.filter((tag) => tag.length > 0)
		)
	]);
	const tagCount = $derived(tagArray.length);

	// valibot 에러 기반으로 상태 확인
	const hasErrors = $derived(Object.values(validationErrors).some((error) => error));

	// 태그 관리 함수들
	let currentTagInput = $state('');

	function addTag() {
		const trimmedTag = currentTagInput.trim();
		if (trimmedTag && tagArray.length < 5 && !tagArray.includes(trimmedTag)) {
			const newTags = [...tagArray, trimmedTag];
			publishData.tags = newTags.join(',');
			currentTagInput = '';
			validateField('tags', publishData.tags);
		}
	}

	function removeTag(tagToRemove: string) {
		const newTags = tagArray.filter((tag) => tag !== tagToRemove);
		publishData.tags = newTags.join(',');
		validateField('tags', publishData.tags);
	}

	function handleTagKeyPress(event: KeyboardEvent) {
		if (event.key === 'Enter' || event.key === ',') {
			event.preventDefault();
			addTag();
		}
	}
</script>

<Button
	onclick={openDialog}
	variant="ghost"
	class="dark:text-mofu-dark-950 dark:hover:bg-mofu bg-mofu flex items-center gap-2 rounded px-4 py-2 text-lg "
	><Icon src={PaperAirplane} class="h-5 w-5" solid />출간하기</Button
>

<Dialog.Root bind:open={isOpen}>
	<Dialog.Content class="dark:bg-mofu-dark-900  text-white sm:max-w-md">
		<Dialog.Header>
			<Dialog.Title>포스트 출간</Dialog.Title>
			<Dialog.Description class="text-mofu-dark-300">포스트 정보를 확인하고 수정한 후 출간하세요.</Dialog.Description>
		</Dialog.Header>

		<div class="space-y-4">
			<div>
				<label for="title" class="text-mofu-dark-100 mb-2 block text-sm font-medium">제목</label>
				<div class="relative">
					<Input
						id="title"
						bind:value={publishData.title}
						placeholder="포스트 제목을 입력하세요"
						class="dark:bg-mofu-dark-700 border-mofu-dark-600 placeholder:text-mofu-dark-400 pr-12 text-white {validationErrors.title
							? 'border-red-500'
							: ''}"
						oninput={() => validateField('title', publishData.title)}
					/>
					<div
						class="absolute top-1/2 right-3 -translate-y-1/2 text-xs {validationErrors.title
							? 'text-red-400'
							: 'text-mofu-dark-400'}"
					>
						{titleCount}/80
					</div>
				</div>
				{#if validationErrors.title}
					<p class="mt-1 text-xs text-rose-400">{validationErrors.title}</p>
				{/if}
			</div>

			<div>
				<label for="slug" class="text-mofu-dark-100 mb-2 block text-sm font-medium">슬러그</label>
				<div class="relative">
					<Input
						id="slug"
						value={publishData.slug}
						oninput={(e) => {
							handleSlugChange(e.currentTarget.value);
							validateField('slug', publishData.slug);
						}}
						placeholder="URL에 사용될 슬러그"
						class="dark:bg-mofu-dark-700 border-mofu-dark-600 placeholder:text-mofu-dark-400 pr-12 text-white {validationErrors.slug
							? 'border-red-500'
							: ''}"
					/>
					<div
						class="absolute top-1/2 right-3 -translate-y-1/2 text-xs {validationErrors.slug
							? 'text-red-400'
							: 'text-mofu-dark-400'}"
					>
						{slugCount}/80
					</div>
				</div>
				{#if validationErrors.slug}
					<p class="mt-1 text-xs text-rose-400">{validationErrors.slug}</p>
				{/if}
				<p class="text-mofu-dark-400 mt-1 text-xs">
					URL: /profile/{userStore.user?.handle || '사용자핸들'}/{publishData.slug}
				</p>
			</div>

			<div>
				<label for="tags" class="text-mofu-dark-100 mb-2 block text-sm font-medium">태그</label>
				<div class="space-y-2">
					<!-- 기존 태그들 표시 -->
					{#if tagArray.length > 0}
						<div class="flex flex-wrap gap-2">
							{#each tagArray as tag}
								<Badge
									variant="default"
									class="hover:bg-mofu-dark-700 bg-mofu-dark-800 text-mofu cursor-pointer"
									onclick={() => removeTag(tag)}
								>
									#{tag}
								</Badge>
							{/each}
						</div>
					{/if}

					<!-- 태그가 5개 미만일 때만 입력 필드 표시 -->
					{#if tagArray.length < 5}
						<div class="relative">
							<Input
								bind:value={currentTagInput}
								onkeydown={handleTagKeyPress}
								placeholder="태그를 입력하고 Enter 또는 쉼표를 누르세요"
								class="dark:bg-mofu-dark-700 border-mofu-dark-600 placeholder:text-mofu-dark-400 pr-12 text-white"
							/>
							<div
								class="absolute top-1/2 right-3 -translate-y-1/2 text-xs {validationErrors.tags
									? 'text-red-400'
									: 'text-mofu-dark-400'}"
							>
								{tagCount}/5
							</div>
						</div>
					{:else}
						<!-- 태그가 5개일 때는 개수만 표시 -->
						<div class="text-xs {validationErrors.tags ? 'text-red-400' : 'text-mofu-dark-400'} text-right">
							{tagCount}/5
						</div>
					{/if}
				</div>
				{#if validationErrors.tags}
					<p class="mt-1 text-xs text-rose-400">{validationErrors.tags}</p>
				{:else}
					<p class="text-mofu-dark-400 mt-1 text-xs">
						태그를 클릭하면 삭제됩니다. Enter 또는 쉼표로 구분하여 입력하세요.
					</p>
				{/if}
			</div>

			<div>
				<label for="summary" class="text-mofu-dark-100 mb-2 block text-sm font-medium">요약 (선택사항)</label>
				<div class="relative">
					<Textarea
						id="summary"
						bind:value={publishData.summary}
						oninput={() => validateField('summary', publishData.summary)}
						placeholder="포스트 요약을 입력하세요"
						class="bg-mofu-dark-700 border-mofu-dark-600 placeholder:text-mofu-dark-400 min-h-[80px] text-white {validationErrors.summary
							? 'border-red-500'
							: ''}"
					/>
					<div
						class="absolute right-2 bottom-2 text-xs {validationErrors.summary ? 'text-red-400' : 'text-mofu-dark-400'}"
					>
						{summaryCount}/500
					</div>
				</div>
				{#if validationErrors.summary}
					<p class="mt-1 text-xs text-rose-400">{validationErrors.summary}</p>
				{/if}
			</div>
		</div>

		<Dialog.Footer>
			<Button
				variant="outline"
				onclick={() => (isOpen = false)}
				class="border-mofu-dark-600 text-mofu-dark-100 hover:bg-mofu-dark-700 bg-transparent"
			>
				취소
			</Button>
			<Button onclick={handlePublish} disabled={isLoading} class="bg-blue-600 hover:bg-blue-700">
				{isLoading ? '출간 중...' : '출간하기'}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import { createPost } from '$lib/api/post/postApi';
	import type { CreatePostRequest } from '$lib/api/post/types';
	import * as v from 'valibot';
	import { createPostSchema } from '$lib/schemas/post';
	import { Icon, PaperAirplane } from 'svelte-hero-icons';

	import TitleInput from './TitleInput.svelte';
	import SlugInput from './SlugInput.svelte';
	import TagsInput from './TagsInput.svelte';
	import SummaryInput from './SummaryInput.svelte';
	import ThumbnailUpload from './ThumbnailUpload.svelte';

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
		tags: '',
		thumbnail: null as string | null,
		thumbnailFile: null as Blob | null
	});
	let validationErrors = $state<Record<string, string>>({});

	function generateSlug(text: string): string {
		return (
			text
				.trim()
				// URL에 안전하지 않은 문자들만 제거
				.replace(/[\s\/\?#\[\]@!$&'()*+,;=]+/g, '-')
				// 연속된 하이픈을 하나로
				.replace(/-+/g, '-')
				// 앞뒤 하이픈 제거
				.replace(/^-+|-+$/g, '')
		);
	}

	function openDialog() {
		publishData = {
			title: title,
			slug: generateSlug(title),
			content: content,
			summary: '',
			tags: tags,
			thumbnail: null,
			thumbnailFile: null
		};
		validationErrors = {};
		isOpen = true;
	}

	function handleValidationChange(field: string) {
		return (error?: string) => {
			validationErrors[field] = error || '';
		};
	}

	function updateField<K extends keyof typeof publishData>(field: K) {
		return (value: (typeof publishData)[K]) => {
			publishData[field] = value;
		};
	}

	function handleThumbnailUpdate(data: { thumbnailFile: Blob; thumbnail: string } | null) {
		if (data) {
			publishData.thumbnail = data.thumbnail;
			publishData.thumbnailFile = data.thumbnailFile;
		} else {
			// Clean up blob URL when removing thumbnail
			if (publishData.thumbnail && publishData.thumbnail.startsWith('blob:')) {
				URL.revokeObjectURL(publishData.thumbnail);
			}
			publishData.thumbnail = null;
			publishData.thumbnailFile = null;
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
				// TODO: 백엔드에서 썸네일 지원 추가되면 활성화
				// thumbnail: publishData.thumbnailFile
			};

			await createPost(postRequest);

			// Clean up thumbnail blob URL after successful post creation
			if (publishData.thumbnail && publishData.thumbnail.startsWith('blob:')) {
				URL.revokeObjectURL(publishData.thumbnail);
			}

			isOpen = false;
			onPublished?.();
		} catch (error) {
			console.error('출간 실패:', error);
			alert('출간에 실패했습니다. 다시 시도해주세요.');
		} finally {
			isLoading = false;
		}
	}

	// Check if there are any validation errors
	const hasErrors = $derived(Object.values(validationErrors).some((error) => error));
</script>

<Button
	onclick={openDialog}
	variant="ghost"
	class="dark:text-mofu-dark-950 dark:hover:bg-mofu bg-mofu flex items-center gap-2 rounded px-4 py-2 text-lg"
>
	<Icon src={PaperAirplane} class="h-5 w-5" solid />
	출간하기
</Button>

<Dialog.Root bind:open={isOpen}>
	<Dialog.Content class="dark:bg-mofu-dark-900 text-white sm:max-w-md">
		<Dialog.Header>
			<Dialog.Title>포스트 출간</Dialog.Title>
			<Dialog.Description class="text-mofu-dark-300">포스트 정보를 확인하고 수정한 후 출간하세요.</Dialog.Description>
		</Dialog.Header>

		<div class="space-y-4">
			<ThumbnailUpload thumbnail={publishData.thumbnail} onUpdate={handleThumbnailUpdate} />
			<TitleInput
				value={publishData.title}
				onUpdate={updateField('title')}
				onValidationChange={handleValidationChange('title')}
			/>

			<SlugInput
				value={publishData.slug}
				onUpdate={updateField('slug')}
				onValidationChange={handleValidationChange('slug')}
			/>

			<TagsInput
				value={publishData.tags}
				onUpdate={updateField('tags')}
				onValidationChange={handleValidationChange('tags')}
			/>

			<SummaryInput
				value={publishData.summary}
				onUpdate={updateField('summary')}
				onValidationChange={handleValidationChange('summary')}
			/>
		</div>

		<Dialog.Footer>
			<Button
				variant="outline"
				onclick={() => (isOpen = false)}
				class="border-mofu-dark-600 text-mofu-dark-100 hover:bg-mofu-dark-700 bg-transparent"
			>
				취소
			</Button>
			<Button
				onclick={handlePublish}
				disabled={isLoading || hasErrors}
				class="bg-blue-600 hover:bg-blue-700 disabled:cursor-not-allowed disabled:opacity-50"
			>
				{isLoading ? '출간 중...' : '출간하기'}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

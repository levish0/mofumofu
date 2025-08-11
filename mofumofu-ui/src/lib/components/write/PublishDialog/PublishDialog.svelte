<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import { createPost, uploadThumbnail } from '$lib/api/post/postApi';
	import type { CreatePostRequest } from '$lib/api/post/types';
	import * as v from 'valibot';
	import { createPostSchema } from '$lib/schemas/post';
	import { Icon, PaperAirplane } from 'svelte-hero-icons';
	import { goto } from '$app/navigation';
	import { userStore } from '$lib/stores/user.svelte';

	import TitleInput from './TitleInput.svelte';
	import SlugInput from './SlugInput.svelte';
	import TagsInput from './TagsInput.svelte';
	import SummaryInput from './SummaryInput.svelte';
	import ThumbnailUpload from './ThumbnailUpload.svelte';
	import { ArrowLeft } from '@lucide/svelte';

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
				summary: publishData.summary.trim() || null,
				hashtags: publishData.tags.trim()
					? publishData.tags
							.split(/[,\n]/)
							.map((tag) => tag.trim())
							.filter((tag) => tag.length > 0)
					: null
			};

			await createPost(postRequest);

			// Upload thumbnail if provided
			if (publishData.thumbnailFile) {
				try {
					await uploadThumbnail({
						slug: publishData.slug.trim(),
						file: new File([publishData.thumbnailFile], 'thumbnail.jpg', { type: publishData.thumbnailFile.type })
					});
				} catch (thumbnailError) {
					console.error('썸네일 업로드 실패:', thumbnailError);
					// 썸네일 업로드 실패는 전체 출간을 취소하지 않음
				}
			}

			// Clean up thumbnail blob URL after successful post creation
			if (publishData.thumbnail && publishData.thumbnail.startsWith('blob:')) {
				URL.revokeObjectURL(publishData.thumbnail);
			}

			isOpen = false;
			onPublished?.();

			// Navigate to the published post
			const userHandle = userStore.user?.handle;
			if (userHandle) {
				await goto(`/@${userHandle}/post/${publishData.slug.trim()}`);
			}
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
	<Dialog.Content class="dark:bg-mofu-dark-900 p-2 text-white sm:max-w-lg">
		<!-- Dialog main content with rounded-b-none -->
		<div class="rounded-t-lg rounded-b-none px-2 pt-4">
			<Dialog.Header class="mb-2 p-0">
				<Dialog.Title>포스트 출간</Dialog.Title>
				<Dialog.Description class="text-mofu-dark-300">포스트 정보를 확인하고 수정한 후 출간하세요.</Dialog.Description>
			</Dialog.Header>

			<div class="hide-scrollbar max-h-[64vh] space-y-4 overflow-y-auto">
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
		</div>

		<!-- Fixed button area -->
		<div class="dark:bg-mofu-dark-900 flex justify-end gap-2 rounded-b-lg px-2 py-2">
			<Button
				variant="ghost"
				onclick={() => (isOpen = false)}
				class="dark:text-mofu-dark-200 text-md flex items-center gap-2 rounded px-4 py-2"
			>
				<ArrowLeft class="h-5 w-5" />
				취소
			</Button>
			<Button
				onclick={handlePublish}
				disabled={isLoading || hasErrors}
				variant="ghost"
				class="dark:text-mofu-dark-950 dark:hover:bg-mofu bg-mofu text-md flex items-center gap-2 rounded px-4 py-2"
			>
				<Icon src={PaperAirplane} class="h-5 w-5" solid />
				{isLoading ? '출간 중...' : '출간하기'}
			</Button>
		</div>
	</Dialog.Content>
</Dialog.Root>

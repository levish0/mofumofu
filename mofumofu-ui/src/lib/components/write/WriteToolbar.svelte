<script lang="ts">
	import {
		Heading1,
		Heading2,
		Heading3,
		Heading4,
		Bold,
		Italic,
		Strikethrough,
		Quote,
		Link,
		Image,
		Code,
		Sigma,
		ChevronUp,
		ChevronDown
	} from '@lucide/svelte';
	import { Button } from '../ui/button';
	import { uploadImage } from '$lib/api/post/postApi';
	import { toast } from 'svelte-sonner';
	import { compressImage } from '$lib/utils/imageCompress';

	interface Props {
		onInsertText: (before: string, after?: string) => void;
		showStickyToolbar: boolean;
		onToggleHeader: () => void;
	}

	const { onInsertText, showStickyToolbar, onToggleHeader }: Props = $props();

	async function handleImageUpload() {
		const input = document.createElement('input');
		input.type = 'file';
		input.accept = 'image/*';

		input.onchange = async (e) => {
			const file = (e.target as HTMLInputElement).files?.[0];
			if (!file) return;

			try {
				toast.loading('이미지 압축 및 업로드 중...');

				// 이미지 압축 (원본 크기 유지)
				const { blob, cleanup } = await compressImage(file, {
					maxFileSizeMB: 8,
					quality: 0.9
				});

				// 압축된 이미지로 새 파일 생성
				const compressedFile = new File([blob], file.name, { type: blob.type });

				// API에 업로드
				const response = await uploadImage({ file: compressedFile });

				// 리소스 정리
				cleanup();

				toast.dismiss();
				toast.success('이미지 업로드 완료!');

				const markdownImage = `![${file.name}](${response.public_url})`;
				onInsertText(markdownImage);
			} catch (error) {
				toast.dismiss();
				toast.error('이미지 업로드 실패');
				console.error('Image upload failed:', error);
			}
		};

		input.click();
	}
</script>

<div class="px-4 pb-4">
	<div class="flex flex-wrap items-center justify-between gap-2">
		<div class="flex flex-wrap items-center gap-2">
			<Button
				variant="ghost"
				onclick={() => onInsertText('# ')}
				class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-400 hover:dark:text-mofu-dark-200 rounded p-2"
			>
				<Heading1 class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('## ')}
				class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-400 hover:dark:text-mofu-dark-200 rounded p-2"
			>
				<Heading2 class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('### ')}
				class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-400 hover:dark:text-mofu-dark-200 rounded p-2"
			>
				<Heading3 class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('#### ')}
				class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-400 hover:dark:text-mofu-dark-200 rounded p-2"
			>
				<Heading4 class="h-5 w-5" />
			</Button>

			<div class="bg-mofu-dark-700 mx-2 h-6 w-px"></div>

			<Button
				variant="ghost"
				onclick={() => onInsertText('**', '**')}
				class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-400 hover:dark:text-mofu-dark-200 rounded p-2 "
				title="굵게"
			>
				<Bold class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('*', '*')}
				class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-400 hover:dark:text-mofu-dark-200 rounded p-2"
				title="기울임"
			>
				<Italic class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('~~', '~~')}
				class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-400 hover:dark:text-mofu-dark-200 rounded p-2"
				title="취소선"
			>
				<Strikethrough class="h-5 w-5" />
			</Button>

			<div class="bg-mofu-dark-700 mx-2 h-6 w-px"></div>

			<Button
				variant="ghost"
				onclick={() => onInsertText('> ')}
				class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-400 hover:dark:text-mofu-dark-200 rounded p-2"
				title="인용"
			>
				<Quote class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('[', '](url)')}
				class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-400 hover:dark:text-mofu-dark-200 rounded p-2"
				title="링크"
			>
				<Link class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={handleImageUpload}
				class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-400 hover:dark:text-mofu-dark-200 rounded p-2"
				title="이미지 업로드"
			>
				<Image class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('$$\n', '\n$$')}
				class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-400 hover:dark:text-mofu-dark-200 rounded p-2"
				title="수식"
			>
				<Sigma class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('```', '\n```')}
				class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-400 hover:dark:text-mofu-dark-200 rounded p-2"
				title="코드"
			>
				<Code class="h-5 w-5" />
			</Button>
		</div>

		<!-- 헤더 토글 버튼 -->
		<Button
			variant="ghost"
			onclick={onToggleHeader}
			class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-400 hover:dark:text-mofu-dark-200 rounded p-2"
			title={showStickyToolbar ? '헤더 보이기' : '헤더 숨기기'}
		>
			{#if showStickyToolbar}
				<ChevronDown class="h-5 w-5" />
			{:else}
				<ChevronUp class="h-5 w-5" />
			{/if}
		</Button>
	</div>
</div>

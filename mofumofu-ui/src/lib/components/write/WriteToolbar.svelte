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
	import ImageCropModal from '../modal/ImageCropModal.svelte';
	import { useImageCrop } from '../settings/PersonalInfoSettings/useImageCrop';

	interface Props {
		onInsertText: (before: string, after?: string) => void;
		showStickyToolbar: boolean;
		onToggleHeader: () => void;
	}

	const { onInsertText, showStickyToolbar, onToggleHeader }: Props = $props();

	let showCrop = $state(false);
	let tempImageSrc = $state('');
	let originalFileName = $state('');

	const { cropImage, cleanupTempImage, handleFileRead } = useImageCrop();

	async function handleImageUpload() {
		const input = document.createElement('input');
		input.type = 'file';
		input.accept = 'image/*';
		
		input.onchange = async (e) => {
			const file = (e.target as HTMLInputElement).files?.[0];
			if (!file) return;

			try {
				originalFileName = file.name;
				tempImageSrc = await handleFileRead(file);
				showCrop = true;
			} catch (error) {
				console.error('Failed to read image file:', error);
				toast.error('이미지 파일을 읽을 수 없습니다');
			}
		};
		
		input.click();
	}

	async function handleCrop(data: { croppedAreaPixels: { x: number; y: number; width: number; height: number } }) {
		try {
			toast.loading('이미지 업로드 중...');
			
			const { blob } = await cropImage(tempImageSrc, data, {
				maxFileSizeMB: 4,
				resizeOptions: { width: 800, height: 600 },
				quality: 0.8
			});

			const file = new File([blob], originalFileName, { type: blob.type });
			const response = await uploadImage({ file });
			
			toast.dismiss();
			toast.success('이미지 업로드 완료!');
			
			const markdownImage = `![${originalFileName}](${response.public_url})`;
			onInsertText(markdownImage);
			
			tempImageSrc = '';
		} catch (error) {
			toast.dismiss();
			toast.error('이미지 업로드 실패');
			console.error('Image upload failed:', error);
		}
	}

	function handleCropCancel() {
		cleanupTempImage(tempImageSrc);
		tempImageSrc = '';
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

<ImageCropModal
	bind:isOpen={showCrop}
	imageSrc={tempImageSrc}
	aspectRatio={1000}
	cropShape="rect"
	onCrop={handleCrop}
	onCancel={handleCropCancel}
/>

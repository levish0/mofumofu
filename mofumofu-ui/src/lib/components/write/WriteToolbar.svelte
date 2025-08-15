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
	import * as m from '../../../paraglide/messages';
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
				toast.loading(m.write_image_uploading());

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
				toast.success(m.write_image_upload_success());

				const markdownImage = `![${file.name}](${response.public_url})`;
				onInsertText(markdownImage);
			} catch (error) {
				toast.dismiss();
				toast.error(m.write_image_upload_error());
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
				title={m.write_toolbar_bold()}
			>
				<Bold class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('*', '*')}
				class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-400 hover:dark:text-mofu-dark-200 rounded p-2"
				title={m.write_toolbar_italic()}
			>
				<Italic class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('~~', '~~')}
				class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-400 hover:dark:text-mofu-dark-200 rounded p-2"
				title={m.write_toolbar_strikethrough()}
			>
				<Strikethrough class="h-5 w-5" />
			</Button>

			<div class="bg-mofu-dark-700 mx-2 h-6 w-px"></div>

			<Button
				variant="ghost"
				onclick={() => onInsertText('> ')}
				class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-400 hover:dark:text-mofu-dark-200 rounded p-2"
				title={m.write_toolbar_quote()}
			>
				<Quote class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('[', '](url)')}
				class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-400 hover:dark:text-mofu-dark-200 rounded p-2"
				title={m.write_toolbar_link()}
			>
				<Link class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={handleImageUpload}
				class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-400 hover:dark:text-mofu-dark-200 rounded p-2"
				title={m.write_toolbar_image_upload()}
			>
				<Image class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('$$\n', '\n$$')}
				class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-400 hover:dark:text-mofu-dark-200 rounded p-2"
				title={m.write_toolbar_math()}
			>
				<Sigma class="h-5 w-5" />
			</Button>
			<Button
				variant="ghost"
				onclick={() => onInsertText('```', '\n```')}
				class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-400 hover:dark:text-mofu-dark-200 rounded p-2"
				title={m.write_toolbar_code()}
			>
				<Code class="h-5 w-5" />
			</Button>
		</div>

		<!-- 헤더 토글 버튼 -->
		<Button
			variant="ghost"
			onclick={onToggleHeader}
			class="hover:dark:bg-mofu-dark-700 dark:text-mofu-dark-400 hover:dark:text-mofu-dark-200 rounded p-2"
			title={showStickyToolbar ? m.write_toolbar_show_header() : m.write_toolbar_hide_header()}
		>
			{#if showStickyToolbar}
				<ChevronDown class="h-5 w-5" />
			{:else}
				<ChevronUp class="h-5 w-5" />
			{/if}
		</Button>
	</div>
</div>

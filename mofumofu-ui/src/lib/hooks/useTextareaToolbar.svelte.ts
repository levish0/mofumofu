// src/lib/hooks/useTextareaToolbar.svelte.ts
import { onMount, onDestroy } from 'svelte';
import { BROWSER } from 'esm-env';

interface TextareaToolbarOptions {
	scrollThreshold?: number;
}

export function useTextareaToolbar(
	textareaElement: HTMLTextAreaElement,
	options: TextareaToolbarOptions = {}
): {
	showStickyToolbar: () => boolean;
} {
	const { scrollThreshold = 50 } = options;

	let showStickyToolbar = $state(false);
	let rafId: number | null = null;

	const handleScroll = () => {
		if (rafId) return;

		rafId = requestAnimationFrame(() => {
			const currentScrollTop = textareaElement.scrollTop;

			// 간단한 로직: 임계값 이상이면 sticky, 이하면 원래대로
			showStickyToolbar = currentScrollTop > scrollThreshold;
			
			rafId = null;
		});
	};

	onMount(() => {
		if (!BROWSER) return;

		// 초기 스크롤 위치 설정
		showStickyToolbar = textareaElement.scrollTop > scrollThreshold;

		textareaElement.addEventListener('scroll', handleScroll, { passive: true });
	});

	onDestroy(() => {
		if (!BROWSER) return;

		textareaElement.removeEventListener('scroll', handleScroll);

		if (rafId) {
			cancelAnimationFrame(rafId);
		}
	});

	return {
		showStickyToolbar: () => showStickyToolbar
	};
}
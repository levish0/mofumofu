// src/lib/hooks/useNavbarScroll.svelte.ts
import { onMount, onDestroy } from 'svelte';
import { BROWSER } from 'esm-env';

interface NavbarScrollOptions {
	navbarHeight?: number;
	scrollThreshold?: number;
}

export function useNavbarScroll(options: NavbarScrollOptions = {}): () => boolean {
	const { navbarHeight = 60, scrollThreshold = 10 } = options;

	let isVisible = $state(true);
	let lastScrollY = $state(0);
	let rafId: number | null = null;

	function updateScrollState() {
		// 브라우저 환경 체크
		if (!BROWSER) return;

		const currentScrollY = window.scrollY;
		const scrollDelta = Math.abs(currentScrollY - lastScrollY);

		// 스크롤 변화가 임계값보다 작으면 무시
		if (scrollDelta < scrollThreshold) return;

		// 맨 위에 있으면 항상 보이기
		if (currentScrollY <= navbarHeight) {
			isVisible = true;
		} else if (currentScrollY > lastScrollY) {
			// 아래로 스크롤: 숨기기
			isVisible = false;
		} else {
			// 위로 스크롤: 보이기
			isVisible = true;
		}

		lastScrollY = currentScrollY;
	}

	const handleScroll = () => {
		if (rafId) return;

		rafId = requestAnimationFrame(() => {
			updateScrollState();
			rafId = null;
		});
	};

	function cleanup() {
		if (BROWSER) {
			window.removeEventListener('scroll', handleScroll);
		}
		if (rafId) {
			cancelAnimationFrame(rafId);
			rafId = null;
		}
	}

	onMount(() => {
		// 초기 스크롤 위치 설정 및 초기 상태 업데이트
		lastScrollY = window.scrollY;
		updateScrollState(); // 초기 가시성 상태 설정

		// 스크롤 이벤트 등록
		window.addEventListener('scroll', handleScroll, { passive: true });
	});

	onDestroy(() => {
		cleanup();
	});

	return () => isVisible;
}

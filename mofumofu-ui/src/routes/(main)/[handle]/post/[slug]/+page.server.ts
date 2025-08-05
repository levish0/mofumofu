import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';

// 더미 포스트 데이터
const dummyPosts = [
	{
		id: '1',
		slug: 'web-animation-guide',
		title: '웹에서 화면이 전환될 때 벌어지는 일을 아시나요? 🤯',
		content: `# 웹에서 화면이 전환될 때 벌어지는 일

> **"SPA에서 페이지 전환은 정말 빠를까?"**  
> 실제로는 *DOM 파괴와 재구성*이 성능 병목을 일으키는 주범일 수 있습니다.

---

## 🧨 전통적인 DOM 전환 방식의 문제

### 1. **기존 DOM 트리 완전 파괴**

SPA에서 라우팅이 발생하면 대부분의 프레임워크는 다음 과정을 거칩니다:

1. 현재 페이지의 DOM을 완전히 제거
2. 새 페이지의 HTML을 렌더링
3. 렌더링 결과를 다시 DOM에 삽입

이 과정은 성능 문제뿐 아니라 **애니메이션 구현에 치명적**입니다.

---

## 🧠 View Transition API가 가져온 패러다임 전환

Chrome에서 실험 중인 [View Transition API](https://developer.chrome.com/docs/web-platform/view-transitions/)는 **DOM 교체 시점과 애니메이션 타이밍을 분리**할 수 있도록 해줍니다.

<img src="https://developer.chrome.com/static/images/view-transitions/view-transitions-overview.png" alt="View Transition Preview" width="600" />

### ✨ 작동 방식:

- 기존 DOM을 스냅샷으로 캡처
- 새 DOM을 백그라운드에서 렌더
- 스냅샷과 새 DOM 사이에 애니메이션 적용
- 완전히 렌더된 뒤 교체

> DOM은 바뀌지만 **사용자는 바뀌는 줄 모르게**.

---

## 🧱 SSGOI 아키텍처 소개

우리가 개발한 **SSGOI (Single State Graph Oriented Infrastructure)**는 프레임워크에 종속되지 않는 구조로 전환 애니메이션을 지원합니다.

### 계층 구조

1. **Core Layer**: DOM 상태 스냅샷 및 뷰 트랜지션 제어
2. **Transition Layer**: 사용자 정의 전환 등록
3. **Adapter Layer**: React / Svelte / Vue 등 프레임워크에 맞게 커스터마이징

### 예시: SvelteKit 연동

---

## 📸 실제 데모 비교

<img src="https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTc9APxkj0xClmrU3PpMZglHQkx446nQPG6lA&amp;s"/>

![](https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTc9APxkj0xClmrU3PpMZglHQkx446nQPG6lA&amp;s)
---

## 🪄 CSS Tricks는 이제 그만

---

## 🧭 결론: 애니메이션도 UX 전략이다

- DOM 파괴 없이 자연스럽게 전환하려면 브라우저 지원이 필요
- View Transition API는 실험적이지만, 점점 많은 프레임워크에서 지원 예정
- 전환이 부자연스럽다면 사용자는 “빠른 웹”이라고 느끼지 않음

> **시각적 연속성이 곧 신뢰입니다.**  
> 우리는 DOM의 존재를 느끼게 하지 않아야 합니다.

---

_더 많은 예제와 코드 샘플은 다음 포스트에서 이어집니다!`,
		created_at: '2025년 7월 26일',
		tags: ['animation', 'frontend', 'ssgoi', 'view transition', 'web', '쓰고'],
		author: {
			handle: 'shiueo',
			name: '더라흔스빌트진오사',
			profile_image: 'https://picsum.photos/40/40?random=1'
		},
		likes: 37,
		views: 142
	}
];

export const load: PageServerLoad = async ({ params }) => {
	if (!params.handle || !params.slug) {
		throw error(404, 'Post not found');
	}

	// Remove @ prefix if present
	const handle = params.handle.startsWith('@') ? params.handle.slice(1) : params.handle;

	// Find dummy post
	const post = dummyPosts.find((p) => p.slug === params.slug && p.author.handle === handle);

	if (!post) {
		throw error(404, 'Post not found');
	}

	return {
		post: post,
		author: post.author,
		handle: params.handle,
		slug: params.slug
	};
};

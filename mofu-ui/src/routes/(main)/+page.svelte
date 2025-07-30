<script lang="ts">
	import PostList from '$lib/components/post/PostList.svelte';

	interface Author {
		name: string;
		avatar: string;
	}

	interface Card {
		id: number;
		image?: string;
		title: string;
		summary: string;
		date: string;
		comments: number;
		views: string;
		author: Author;
		likes: number;
	}

	// 랜덤 제목과 요약 생성을 위한 배열들
	const titles = [
		'웹 개발의 최신 트렌드',
		'React vs Svelte 비교 분석',
		'TypeScript 활용법',
		'CSS Grid와 Flexbox',
		'JavaScript 성능 최적화',
		'모던 프론트엔드 개발',
		'API 설계 베스트 프랙티스',
		'데이터베이스 최적화 방법',
		'클라우드 서비스 활용',
		'DevOps와 CI/CD',
		'보안 취약점 방어하기',
		'UX/UI 디자인 원칙',
		'반응형 웹 디자인',
		'웹 접근성 가이드라인',
		'마이크로서비스 아키텍처',
		'GraphQL vs REST API',
		'머신러닝과 웹개발',
		'블록체인 기술 이해하기',
		'PWA 개발 가이드',
		'웹 성능 측정과 개선'
	];

	const summaries = [
		'최신 웹 개발 트렌드와 기술들을 알아보고, 실무에서 어떻게 활용할 수 있는지 살펴보겠습니다.',
		'프론트엔드 개발에서 고려해야 할 다양한 요소들과 최적화 방법을 상세히 다룹니다.',
		'개발 효율성을 높이고 코드 품질을 개선하는 방법들을 실제 예제와 함께 설명합니다.',
		'사용자 경험을 향상시키기 위한 디자인 패턴과 구현 방법을 알아봅니다.',
		'성능 최적화와 메모리 관리, 그리고 디버깅 기법에 대해 깊이 있게 다룹니다.',
		'현대적인 개발 환경 구축과 협업 도구 활용법을 단계별로 안내합니다.',
		'확장 가능하고 유지보수하기 쉬운 코드 작성을 위한 아키텍처 설계 방법론입니다.',
		'보안과 성능을 모두 고려한 안정적인 웹 애플리케이션 개발 가이드입니다.',
		'다양한 플랫폼과 디바이스에서 일관된 사용자 경험을 제공하는 방법을 알아봅니다.',
		'팀 협업과 코드 리뷰, 그리고 효과적인 문서화 방법에 대해 논의합니다.'
	];

	const authors = [
		'김개발',
		'이프론트',
		'박백엔드',
		'최디자인',
		'정보안',
		'한데이터',
		'송클라우드',
		'임모바일',
		'조풀스택',
		'윤데브옵스',
		'장아키텍트',
		'노시니어'
	];

	const dates = [
		'1시간 전',
		'3시간 전',
		'1일 전',
		'2일 전',
		'3일 전',
		'1주 전',
		'2주 전',
		'3주 전',
		'1개월 전',
		'2개월 전',
		'3개월 전',
		'6개월 전'
	];

	// 더미 데이터 생성 함수 (완전 랜덤)
	const createDummyCard = (id: number): Card => {
		const randomTitle = titles[Math.floor(Math.random() * titles.length)];
		const randomSummary = summaries[Math.floor(Math.random() * summaries.length)];
		const randomAuthor = authors[Math.floor(Math.random() * authors.length)];
		const randomDate = dates[Math.floor(Math.random() * dates.length)];

		return {
			id,
			image: Math.random() > 0.7 ? `https://picsum.photos/400/225?random=${id}` : undefined,
			title: `${randomTitle} ${id}`,
			summary: randomSummary,
			date: randomDate,
			comments: Math.floor(Math.random() * 100),
			views: 'Views',
			author: {
				name: randomAuthor,
				avatar: `https://picsum.photos/32/32?random=${id + 1000}`
			},
			likes: Math.floor(Math.random() * 200)
		};
	};

	// 초기 데이터 (Svelte 5 $state 사용)
	let cards = $state([
		{
			id: 1,
			title: 'Lorem ipsum dolor sit amet 1',
			summary:
				'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.',
			date: '4월 전',
			comments: 11,
			views: 'Views',
			author: {
				name: 'Neo',
				avatar: 'https://picsum.photos/32/32?random=1'
			},
			likes: 34
		},
		{
			id: 2,
			title: 'Consectetur adipiscing elit 2',
			summary:
				'Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.',
			date: '6월 전',
			comments: 35,
			views: 'Views',
			author: {
				name: 'Luna',
				avatar: 'https://picsum.photos/32/32?random=2'
			},
			likes: 49
		},
		{
			id: 3,
			image: 'https://github.com/levish0/mofumofu/raw/main/assets/mofumofu_stroke.png',
			title: '모후모후 만들기',
			summary:
				'Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.',
			date: '2025년 7월 21일',
			comments: 1,
			views: 'Views',
			author: {
				name: 'Kai',
				avatar: 'https://picsum.photos/32/32?random=3'
			},
			likes: 39
		},
		{
			id: 4,
			image: 'https://picsum.photos/400/225?random=4',
			title: 'Incididunt ut labore et dolore 4',
			summary:
				'Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt.',
			date: '2025년 7월 19일',
			comments: 4,
			views: 'Views',
			author: {
				name: 'Mina',
				avatar: 'https://picsum.photos/32/32?random=4'
			},
			likes: 33
		},
		{
			id: 5,
			image: 'https://picsum.photos/400/225?random=5',
			title: 'Magna aliqua ut enim ad minim 5',
			summary:
				'Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, sed quia non numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat voluptatem.',
			date: '3월 전',
			comments: 15,
			views: 'Views',
			author: {
				name: 'Jay',
				avatar: 'https://picsum.photos/32/32?random=5'
			},
			likes: 41
		},
		{
			id: 6,
			image: 'https://picsum.photos/400/225?random=6',
			title: 'Veniam quis nostrud 6',
			summary:
				'Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur?',
			date: '2월 전',
			comments: 9,
			views: 'Views',
			author: {
				name: 'Rin',
				avatar: 'https://picsum.photos/32/32?random=6'
			},
			likes: 27
		},
		{
			id: 7,
			image: 'https://picsum.photos/400/225?random=7',
			title: 'Exercitation ullamco laboris 7',
			summary:
				'Quis autem vel eum iure reprehenderit qui in ea voluptate velit esse quam nihil molestiae consequatur, vel illum qui dolorem eum fugiat quo voluptas nulla pariatur?',
			date: '1년 전',
			comments: 22,
			views: 'Views',
			author: {
				name: 'Alex',
				avatar: 'https://picsum.photos/32/32?random=7'
			},
			likes: 65
		},
		{
			id: 8,
			image: 'https://picsum.photos/400/225?random=8',
			title: 'Nisi ut aliquip ex ea 8',
			summary:
				'At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident.',
			date: '5일 전',
			comments: 7,
			views: 'Views',
			author: {
				name: 'Sophia',
				avatar: 'https://picsum.photos/32/32?random=8'
			},
			likes: 59
		}
	]);

	// 상태 관리
	let loading = $state(false);
	let currentPage = $state(1);
	const PAGE_SIZE = 8;
	const MAX_POSTS = 30; // 테스트를 위해 50개로 제한
	const skeletonCount = PAGE_SIZE-4;
	let hasMore = $state(true);

	// 더 많은 포스트 로드 (50개 제한)
	const onLoadMore = async () => {
		if (loading || !hasMore) return;

		// 이미 최대 개수에 도달했는지 확인
		if (cards.length >= MAX_POSTS) {
			hasMore = false;
			return;
		}

		loading = true;

		// API 호출 시뮬레이션 (500ms~1.5초 랜덤 딜레이)
		const delay = Math.random() * 500 + 100;
		await new Promise((resolve) => setTimeout(resolve, delay));

		const newCards: Card[] = [];
		let nextId = 1;

		// 기존 카드들 중 최대 id 찾기
		if (cards.length > 0) {
			nextId = Math.max(...cards.map((c) => c.id)) + 1;
		}

		// 남은 개수만큼만 로드
		const remainingCount = MAX_POSTS - cards.length;
		const loadCount = Math.min(PAGE_SIZE, remainingCount);

		for (let i = 0; i < loadCount; i++) {
			newCards.push(createDummyCard(nextId + i));
		}

		cards = [...cards, ...newCards];
		currentPage++;

		// 최대 개수에 도달했으면 hasMore를 false로 설정
		if (cards.length >= MAX_POSTS) {
			hasMore = false;
		}

		loading = false;
	};
</script>

<PostList {cards} {loading} {onLoadMore} {hasMore} {skeletonCount} />

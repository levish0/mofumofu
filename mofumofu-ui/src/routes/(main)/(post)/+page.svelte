<script lang="ts">
	import PostList from '$lib/components/post/PostList.svelte';
	import { getPosts } from '$lib/api/post/postApi';
	import type { PostListItem, PostSortOrder } from '$lib/api/post/types';
	import { onMount } from 'svelte';

	interface Author {
		name: string;
		avatar: string;
	}

	interface Card {
		image?: string;
		title: string;
		summary: string;
		date: string;
		comments: number;
		views: string;
		author: Author;
		likes: number;
		slug: string;
		handle: string;
	}

	// PostListItem을 Card 형태로 변환하는 함수
	const convertToCard = (post: PostListItem): Card => {
		const formatDate = (dateStr: string) => {
			const date = new Date(dateStr);
			const now = new Date();
			const diff = now.getTime() - date.getTime();
			const days = Math.floor(diff / (1000 * 60 * 60 * 24));
			
			if (days === 0) {
				const hours = Math.floor(diff / (1000 * 60 * 60));
				return hours === 0 ? '방금 전' : `${hours}시간 전`;
			} else if (days < 7) {
				return `${days}일 전`;
			} else if (days < 30) {
				const weeks = Math.floor(days / 7);
				return `${weeks}주 전`;
			} else if (days < 365) {
				const months = Math.floor(days / 30);
				return `${months}개월 전`;
			} else {
				const years = Math.floor(days / 365);
				return `${years}년 전`;
			}
		};

		return {
			image: post.thumbnail_image || undefined,
			title: post.title,
			summary: post.summary || '',
			date: formatDate(post.created_at),
			comments: post.comment_count,
			views: 'Views',
			author: {
				name: post.user_name,
				avatar: post.user_avatar || `https://picsum.photos/32/32?random=${post.user_handle}`
			},
			likes: post.like_count,
			slug: post.slug,
			handle: post.user_handle
		};
	};

	// 상태 관리
	let cards = $state<Card[]>([]);
	let loading = $state(false);
	let initialLoading = $state(true);
	let currentPage = $state(1);
	const PAGE_SIZE = 8;
	const skeletonCount = PAGE_SIZE - 4;
	let hasMore = $state(true);
	const sortOrder: PostSortOrder = 'latest';

	// 초기 데이터 로드
	const loadInitialPosts = async () => {
		try {
			initialLoading = true;
			const response = await getPosts({
				page: 1,
				page_size: PAGE_SIZE,
				sort: sortOrder
			});

			cards = response.posts.map(convertToCard);
			hasMore = response.has_more;
			currentPage = 1;
		} catch (error) {
			console.error('Failed to load initial posts:', error);
			// 에러 시 빈 배열로 초기화
			cards = [];
			hasMore = false;
		} finally {
			initialLoading = false;
		}
	};

	// 더 많은 포스트 로드
	const onLoadMore = async () => {
		if (loading || !hasMore) return;

		loading = true;

		try {
			const response = await getPosts({
				page: currentPage + 1,
				page_size: PAGE_SIZE,
				sort: sortOrder
			});

			const newCards = response.posts.map(convertToCard);
			cards = [...cards, ...newCards];
			currentPage++;
			hasMore = response.has_more;
		} catch (error) {
			console.error('Failed to load more posts:', error);
		} finally {
			loading = false;
		}
	};

	// 컴포넌트 마운트 시 초기 데이터 로드
	onMount(() => {
		loadInitialPosts();
	});
</script>

<PostList {cards} {loading} {onLoadMore} {hasMore} {skeletonCount} />

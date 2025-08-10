import type { PostListItem, PostSortOrder } from '$lib/api/post/types';

interface PostsFilter {
	keyword: string;
	tags: string[];
	sortBy: string;
	timeRange: string;
}

interface PostsState {
	posts: PostListItem[];
	currentPage: number;
	targetPage: number;
	hasMore: boolean;
	loading: boolean;
	initialLoading: boolean;
	sortOrder: PostSortOrder;
	filter: PostsFilter;
	isSearchMode: boolean; // 검색 모드인지 일반 모드인지
}

let state = $state<PostsState>({
	posts: [],
	currentPage: 1,
	targetPage: 1,
	hasMore: true,
	loading: false,
	initialLoading: true,
	sortOrder: 'latest',
	filter: {
		keyword: '',
		tags: [],
		sortBy: 'recent',
		timeRange: 'all'
	},
	isSearchMode: false
});

export const postsStore = {
	get posts() {
		return state.posts;
	},
	get currentPage() {
		return state.currentPage;
	},
	get targetPage() {
		return state.targetPage;
	},
	get hasMore() {
		return state.hasMore;
	},
	get loading() {
		return state.loading;
	},
	get initialLoading() {
		return state.initialLoading;
	},
	get sortOrder() {
		return state.sortOrder;
	},
	get filter() {
		return state.filter;
	},
	get isSearchMode() {
		return state.isSearchMode;
	},

	setPosts(posts: PostListItem[]) {
		state.posts = posts;
	},
	addPosts(posts: PostListItem[]) {
		state.posts = [...state.posts, ...posts];
	},
	setCurrentPage(page: number) {
		state.currentPage = page;
	},
	setTargetPage(page: number) {
		state.targetPage = page;
	},
	setHasMore(hasMore: boolean) {
		state.hasMore = hasMore;
	},
	setLoading(loading: boolean) {
		state.loading = loading;
	},
	setInitialLoading(loading: boolean) {
		state.initialLoading = loading;
	},
	setSortOrder(order: PostSortOrder) {
		state.sortOrder = order;
	},
	setFilter(filter: Partial<PostsFilter>) {
		state.filter = { ...state.filter, ...filter };
		// 검색어나 태그가 있으면 검색 모드로 전환
		state.isSearchMode = !!(state.filter.keyword || state.filter.tags.length > 0);
	},
	setSearchMode(isSearchMode: boolean) {
		state.isSearchMode = isSearchMode;
	},
	resetFilter() {
		state.filter = {
			keyword: '',
			tags: [],
			sortBy: 'recent',
			timeRange: 'all'
		};
		state.isSearchMode = false;
	},

	reset() {
		state.posts = [];
		state.currentPage = 1;
		state.targetPage = 1;
		state.hasMore = true;
		state.loading = false;
		state.initialLoading = true;
	}
};

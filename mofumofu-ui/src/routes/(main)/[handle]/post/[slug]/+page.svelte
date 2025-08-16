<script lang="ts">
	import '$lib/styles/markdown.css';
	import { getContext, onMount } from 'svelte';
	import type { PageData } from './$types';
	import { Heart, Icon, EllipsisVertical, PencilSquare, Trash } from 'svelte-hero-icons';
	import { incrementPostView } from '$lib/api/post/postApi';
	import { userStore } from '$lib/stores/user.svelte';
	import { fly } from 'svelte/transition';
	import { Button } from '$lib/components/ui/button';
	import { goto } from '$app/navigation';
	import { deletePost } from '$lib/api/post/postApi';
	import * as Dialog from '$lib/components/ui/dialog';
	import { toast } from 'svelte-sonner';
	import FollowButton from '$lib/components/profile/FollowButton.svelte';
	import { Badge } from '$lib/components/ui/badge';
	import { createLike, deleteLike, checkLikeStatus } from '$lib/api/like/likeApi';
	import { authStore } from '$lib/stores/auth.svelte';
	import FloatingTOC from '$lib/components/post/FloatingTOC.svelte';
	import FloatingNavigation from '$lib/components/post/FloatingNavigation.svelte';

	const { data }: { data: PageData } = $props();

	// 백엔드에서 렌더된 HTML과 TOC 사용
	const htmlContent = data.post.rendered;
	const tocItems = data.post.toc_items;

	type NavbarContext = {
		isVisible: () => boolean;
		isAtTop: () => boolean;
	};

	const navbar = getContext<NavbarContext>('navbar');
	const topPosition = $derived(navbar.isVisible() ? '68px' : '8px');

	// 작성자 확인 및 드롭다운 상태
	const currentUser = $derived(userStore.user);
	const isAuthor = $derived(currentUser?.handle === data.author.handle);
	let isDropdownOpen = $state(false);
	let isDeleteModalOpen = $state(false);
	let isDeleting = $state(false);
	let closeTimer: ReturnType<typeof setTimeout> | null = null;

	// Like 관련 상태
	let isLiked = $state(false);
	let likeCount = $state(data.post.like_count || 0);
	let isLikeLoading = $state(true);
	let isLikeSubmitting = $state(false);

	function openDropdown() {
		if (closeTimer) {
			clearTimeout(closeTimer);
			closeTimer = null;
		}
		isDropdownOpen = true;
	}

	function scheduleClose() {
		closeTimer = setTimeout(() => {
			isDropdownOpen = false;
			closeTimer = null;
		}, 100);
	}

	function handleEdit() {
		isDropdownOpen = false;
		goto(`/edit/${data.slug}`);
	}

	function handleDelete() {
		isDropdownOpen = false;
		isDeleteModalOpen = true;
	}

	async function confirmDelete() {
		try {
			isDeleting = true;
			await deletePost({ slug: data.slug });
			toast.success('포스트가 성공적으로 삭제되었습니다.');
			// 삭제 후 사용자 프로필 페이지로 이동
			goto(`/@${data.author.handle}/profile`);
		} catch (error) {
			console.error('Failed to delete post:', error);
			toast.error('포스트 삭제에 실패했습니다. 다시 시도해주세요.');
		} finally {
			isDeleting = false;
		}
	}

	function cancelDelete() {
		isDeleteModalOpen = false;
	}

	// Like 상태 확인
	async function loadLikeStatus() {
		if (!authStore.isAuthenticated) {
			isLikeLoading = false;
			return;
		}

		try {
			const response = await checkLikeStatus({
				handle: data.author.handle,
				slug: data.post.slug
			});
			isLiked = response.is_liked;
		} catch (error) {
			console.error('Failed to check like status:', error);
		} finally {
			isLikeLoading = false;
		}
	}

	// Like 토글
	async function toggleLike() {
		if (!authStore.isAuthenticated) {
			goto('/account/signin');
			return;
		}

		if (isLikeSubmitting) return;

		isLikeSubmitting = true;
		try {
			if (isLiked) {
				await deleteLike({
					handle: data.author.handle,
					slug: data.post.slug
				});
				isLiked = false;
				likeCount = Math.max(0, likeCount - 1);
			} else {
				await createLike({
					handle: data.author.handle,
					slug: data.post.slug
				});
				isLiked = true;
				likeCount += 1;
			}
		} catch (error) {
			console.error('Failed to toggle like:', error);
			toast.error('좋아요 처리에 실패했습니다');
		} finally {
			isLikeSubmitting = false;
		}
	}

	onMount(() => {
		incrementPostView({ handle: data.handle, slug: data.slug }).catch((error) => {
			console.warn('Failed to increment view count:', error);
		});

		// Like 상태 로드
		loadLikeStatus();
	});
</script>

<svelte:head>
	<title>{data.post.title} - {data.author.name} - Mofumofu</title>
	<meta name="description" content={data.post.summary || data.post.title} />

	<!-- Open Graph -->
	<meta property="og:title" content="{data.post.title} - {data.author.name}" />
	<meta property="og:description" content={data.post.summary || data.post.title} />
	<meta property="og:type" content="article" />
	<meta property="og:url" content="https://mofumofu.ink/{data.author.handle}/post/{data.post.slug}" />
	<meta property="og:image" content="https://mofumofu.ink/og-default.png" />
	<meta property="og:site_name" content="Mofumofu" />
	<meta property="article:author" content={data.author.name} />
	<meta property="article:published_time" content={data.post.created_at} />
	{#each data.post.tags as tag}
		<meta property="article:tag" content={tag} />
	{/each}

	<!-- Twitter Card -->
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:title" content="{data.post.title} - {data.author.name}" />
	<meta name="twitter:description" content={data.post.summary || data.post.title} />
	<meta name="twitter:image" content="https://mofumofu.ink/og-default.png" />
</svelte:head>

<div class="relative min-h-screen pt-2">
	<div class="max-w-8xl mx-auto gap-4 px-4">
		<div class="flex gap-4">
			<!-- Left Column: Post Content -->
			<div class="min-w-0 flex-1">
				<article class="rounded-lg">
					<!-- Post Header -->
					<header>
						<h1 class="dark:text-mofu-200 mb-4 text-4xl font-bold break-all">
							{data.post.title}
						</h1>

						<!-- Author Info -->
						<div class="mb-4 flex items-center justify-between">
							<a
								href="/@{data.author.handle}/profile"
								class="flex items-center gap-4 transition-opacity hover:opacity-80"
							>
								{#if data.author.profile_image}
									<img
										src={data.author.profile_image}
										alt={data.author.name}
										class="h-12 w-12 rounded-full object-cover"
									/>
								{:else}
									<div
										class="dark:bg-mofu-dark-600 flex h-12 w-12 items-center justify-center rounded-full bg-gray-100"
									>
										<span class="dark:text-mofu-dark-200 text-lg font-medium text-gray-600">
											{data.author.name.charAt(0).toUpperCase()}
										</span>
									</div>
								{/if}
								<div>
									<p class="font-medium text-gray-900 dark:text-white">{data.author.name}</p>
									<p class="dark:text-mofu-dark-400 text-sm text-gray-500">
										{data.post.created_at}
									</p>
								</div>
							</a>

							<div class="flex items-center gap-2">
								<!-- Like Button -->
								{#if isLikeLoading}
									<div class="flex items-center gap-2 rounded-full px-4 py-2">
										<div class="h-5 w-5 animate-pulse rounded bg-gray-300 dark:bg-gray-600"></div>
										<span class="text-sm">{likeCount}</span>
									</div>
								{:else}
									<button
										onclick={toggleLike}
										disabled={isLikeSubmitting}
										class="flex items-center gap-2 rounded-full px-4 py-2 transition-colors {isLiked
											? 'text-rose-600 dark:text-rose-500'
											: 'dark:text-mofu-dark-400 text-mofu-light-800 hover:text-rose-600 dark:hover:text-rose-500'} 
										{isLikeSubmitting ? 'cursor-not-allowed opacity-50' : ''}"
									>
										<Icon src={Heart} class="h-5 w-5" solid />
										<span class="text-sm">{likeCount}</span>
									</button>
								{/if}

								{#if isAuthor}
									<!-- Author Options Dropdown -->
									<div
										class="relative"
										role="button"
										tabindex="0"
										onmouseenter={openDropdown}
										onmouseleave={scheduleClose}
									>
										<Button variant="ghost" class="dark:text-mofu-dark-400 h-9 w-9 p-2">
											<Icon src={EllipsisVertical} class="h-5 w-5" />
										</Button>

										{#if isDropdownOpen}
											<div
												class="dark:bg-mofu-dark-800 absolute top-12 right-0 z-50 w-48 rounded-lg bg-white text-sm font-bold shadow-lg"
												transition:fly={{ y: -8, duration: 150 }}
												style="transform-origin: top right;"
											>
												<div class="py-1">
													<button
														class="dark:text-mofu-dark-200 text-mofu-light-800 flex w-full items-center px-4 py-2 hover:opacity-70"
														onclick={handleEdit}
													>
														<Icon src={PencilSquare} solid size="16" class="mr-3" />
														수정하기
													</button>
													<button
														class="flex w-full items-center px-4 py-2 text-rose-600 hover:opacity-70 dark:text-rose-500"
														onclick={handleDelete}
													>
														<Icon src={Trash} solid size="16" class="mr-3" />
														삭제하기
													</button>
												</div>
											</div>
										{/if}
									</div>
								{:else}
									<!-- Follow Button -->
									<FollowButton handle={data.author.handle} />
								{/if}
							</div>
						</div>

						<!-- Tags -->
						<div class="flex flex-wrap gap-2">
							{#each data.post.tags as tag}
								<Badge class="bg-mofu/10 text-mofu rounded-full px-2 py-1 text-xs font-medium">
									#{tag}
								</Badge>
							{/each}
						</div>
					</header>

					<!-- Post Content -->
					<div class="prose prose-invert prose-lg text-mofu-dark-200 max-w-none break-all">
						{@html htmlContent}
					</div>
				</article>
			</div>

			<!-- Right Column: TOC (데스크톱에서만 표시) -->
			<div class="border-mofu-dark-800 hidden w-80 flex-shrink-0 border-l md:block">
				<div class="sticky transition-all duration-100 ease-out" style="top: {topPosition}">
					<div class="px-4">
						<h3 class="dark:text-mofu-dark-200 mb-2 text-xl font-semibold">목차</h3>
						<nav class="max-h-[80vh] space-y-2 overflow-y-auto">
							{#each tocItems as item}
								<a
									href="#{item.id}"
									class="dark:text-mofu-dark-300 block text-sm text-gray-600 transition-colors hover:text-gray-900 dark:hover:text-white"
									style="padding-left: {(item.level - 1) * 12}px"
								>
									{item.text}
								</a>
							{/each}
						</nav>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>

<!-- 모바일용 플로팅 컴포넌트들 -->
<FloatingTOC {tocItems} />
<FloatingNavigation />

<!-- 삭제 확인 Dialog -->
<Dialog.Root bind:open={isDeleteModalOpen}>
	<Dialog.Content class="dark:bg-mofu-dark-800 p-2 text-black sm:max-w-md dark:text-white">
		<div class="rounded-lg px-2 pt-4">
			<Dialog.Header class="mb-2 p-0">
				<Dialog.Title class="text-lg font-semibold">포스트 삭제</Dialog.Title>
				<Dialog.Description class="text-gray-600 dark:text-gray-300">
					이 포스트를 정말 삭제하시겠습니까?<br />
					삭제된 포스트는 복구할 수 없습니다.
				</Dialog.Description>
			</Dialog.Header>
		</div>

		<!-- 버튼 영역 -->
		<div class="flex justify-end gap-3 rounded-b-lg px-2 py-2">
			<Button variant="ghost" onclick={cancelDelete} disabled={isDeleting}>취소</Button>
			<Button variant="destructive" onclick={confirmDelete} disabled={isDeleting}>
				{isDeleting ? '삭제 중...' : '삭제하기'}
			</Button>
		</div>
	</Dialog.Content>
</Dialog.Root>

<!-- 삭제 중 로딩 오버레이 -->
{#if isDeleting}
	<div class="fixed inset-0 z-100 flex items-center justify-center bg-black/50 backdrop-blur-sm">
		<div class="flex flex-col items-center space-y-4">
			<div class="border-mofu h-12 w-12 animate-spin rounded-full border-4 border-t-transparent"></div>
			<p class="text-lg font-medium text-white">삭제 중...</p>
		</div>
	</div>
{/if}

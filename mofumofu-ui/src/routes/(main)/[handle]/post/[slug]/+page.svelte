<script lang="ts">
	import '$lib/styles/markdown.css';
	import { getContext } from 'svelte';
	import type { PageData } from './$types';
	import { processMarkdown, type TocItem } from '$lib/utils/markdown';
	import { Heart, Icon } from 'svelte-hero-icons';

	const { data }: { data: PageData } = $props();

	let htmlContent = $state('');
	let tocItems: TocItem[] = $state([]);

	type NavbarContext = {
		isVisible: () => boolean;
		isAtTop: () => boolean;
	};

	const navbar = getContext<NavbarContext>('navbar');
	const topPosition = $derived(navbar.isVisible() ? '68px' : '8px');

	$effect(() => {
		(async () => {
			const result = await processMarkdown(data.post.content);
			htmlContent = result.htmlContent;
			tocItems = result.tocItems;
		})();
	});
</script>

<svelte:head>
	<title>{data.post.title} - {data.author.name} - Mofumofu</title>
	<meta name="description" content={data.post.content.slice(0, 160)} />

	<!-- Open Graph -->
	<meta property="og:title" content="{data.post.title} - {data.author.name}" />
	<meta property="og:description" content={data.post.content.slice(0, 160)} />
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
	<meta name="twitter:description" content={data.post.content.slice(0, 160)} />
	<meta name="twitter:image" content="https://mofumofu.ink/og-default.png" />
</svelte:head>

<div class="relative min-h-screen pt-2">
	<div class="max-w-8xl mx-auto gap-4 px-4">
		<div class="flex gap-4">
			<!-- Left Column: Post Content -->
			<div class="min-w-0 flex-1">
				<article class="rounded-lg">
					<!-- Post Header -->
					<header class=" mb-8">
						<h1 class="dark:text-mofu-200 mb-4 text-4xl font-bold">
							{data.post.title}
						</h1>

						<!-- Author Info -->
						<div class="mb-4 flex items-center justify-between">
							<div class="flex items-center gap-4">
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
							</div>

							<div class="flex items-center gap-3">
								<!-- Like Button -->
								<button
									class="text-mofu-dark-300 flex items-center gap-2 rounded-full px-4 py-2 transition-colors hover:text-rose-600 dark:hover:text-red-400"
								>
									<Icon src={Heart} class="h-5 w-5" solid />
									<span class="text-sm">{data.post.like_count || 0}</span>
								</button>

								<!-- Follow Button -->
								<button
									class="rounded-full bg-green-500 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-green-600"
								>
									팔로우
								</button>
							</div>
						</div>

						<!-- Tags -->
						<div class="mb-4 flex flex-wrap gap-2">
							{#each data.post.tags as tag}
								<span
									class="rounded-full border border-green-200 bg-green-100 px-3 py-1 text-sm text-green-800 dark:border-green-800 dark:bg-green-900/20 dark:text-green-300"
								>
									{tag}
								</span>
							{/each}
						</div>
					</header>

					<!-- Post Content -->
					<div class="prose prose-invert prose-lg text-mofu-dark-200 max-w-none">
						{@html htmlContent}
					</div>
				</article>
			</div>

			<!-- Right Column: TOC -->
			<div class="border-mofu-dark-800 w-80 flex-shrink-0 border-l">
				<div class="sticky transition-all duration-100 ease-out" style="top: {topPosition}">
					<div class="px-4">
						<h3 class="dark:text-mofu-dark-200 mb-2 text-xl font-semibold">목차</h3>
						<nav class="space-y-2">
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

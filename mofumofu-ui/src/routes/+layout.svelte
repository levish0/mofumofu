<script lang="ts">
	import '../app.css';
	import { ModeWatcher } from 'mode-watcher';

	let { children } = $props();
</script>

<svelte:head>
	<title>Mofumofu - A soft and simple blogging platform for everyone.</title>
	<meta
		name="description"
		content="Mofumofu는 새로운 소셜 미디어 플랫폼입니다. 다양한 사람들과 소통하고 콘텐츠를 공유하세요."
	/>
	<meta name="keywords" content="소셜미디어, SNS, 커뮤니티, 소통, 콘텐츠" />

	<!-- Open Graph -->
	<meta property="og:title" content="Mofumofu - 소셜 미디어 플랫폼" />
	<meta
		property="og:description"
		content="Mofumofu는 새로운 소셜 미디어 플랫폼입니다. 다양한 사람들과 소통하고 콘텐츠를 공유하세요."
	/>
	<meta property="og:type" content="website" />
	<meta property="og:url" content="https://mofumofu.ink" />
	<meta property="og:image" content="https://mofumofu.ink/og-default.png" />
	<meta property="og:site_name" content="Mofumofu" />

	<!-- Twitter Card -->
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:title" content="Mofumofu - 소셜 미디어 플랫폼" />
	<meta
		name="twitter:description"
		content="Mofumofu는 새로운 소셜 미디어 플랫폼입니다. 다양한 사람들과 소통하고 콘텐츠를 공유하세요."
	/>
	<meta name="twitter:image" content="https://mofumofu.ink/og-default.png" />

	<!-- Additional SEO -->
	<meta name="robots" content="index, follow" />
	<meta name="author" content="Mofumofu Team" />
	<link rel="canonical" href="https://mofumofu.ink" />
</svelte:head>

<ModeWatcher defaultMode="dark" />
<div class="dark:bg-mofu-dark-900 font-pretendard min-h-screen max-w-screen">
	<main>
		{@render children()}
	</main>
</div>

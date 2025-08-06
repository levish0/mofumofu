<script lang="ts">
	import '../app.css';
	import { ModeWatcher } from 'mode-watcher';

	let { children } = $props();
</script>

<svelte:head>
	<title>Mofumofu - A soft and simple blogging platform for everyone.</title>
	<meta
		name="description"
		content="Mofumofu is a new social media platform. Connect with diverse people and share content."
	/>
	<meta name="keywords" content="social media, SNS, community, communication, content" />

	<!-- Open Graph -->
	<meta property="og:title" content="Mofumofu - Social Media Platform" />
	<meta
		property="og:description"
		content="Mofumofu is a new social media platform. Connect with diverse people and share content."
	/>
	<meta property="og:type" content="website" />
	<meta property="og:url" content="https://mofumofu.ink" />
	<meta property="og:image" content="https://mofumofu.ink/og-default.png" />
	<meta property="og:site_name" content="Mofumofu" />

	<!-- Twitter Card -->
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:title" content="Mofumofu - Social Media Platform" />
	<meta
		name="twitter:description"
		content="Mofumofu is a new social media platform. Connect with diverse people and share content."
	/>
	<meta name="twitter:image" content="https://mofumofu.ink/og-default.png" />

	<!-- Additional SEO -->
	<meta name="robots" content="index, follow" />
	<meta name="author" content="Mofumofu Team" />
	<link rel="canonical" href="https://mofumofu.ink" />
</svelte:head>

<ModeWatcher defaultMode="system" />
<div class="dark:bg-mofu-dark-900 font-pretendard bg-mofu-light-100 min-h-screen max-w-screen">
	<main>
		{@render children()}
	</main>
</div>

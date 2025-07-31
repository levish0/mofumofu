<script lang="ts">
	import '../app.css';
	import { ModeWatcher } from 'mode-watcher';

	let { children } = $props();
</script>

<ModeWatcher defaultMode="dark" />
<div class="dark:bg-mofu-dark-900 font-pretendard max-w-screen">
	{@render children()}
</div>

<script lang="ts">
	import '../app.css';
	import { ModeWatcher } from 'mode-watcher';

	let { children } = $props();
</script>

<ModeWatcher defaultMode="dark" />
<div class="font-pretendard bg-mofu-dark-900 h-screen w-screen text-white">
	{@render children()}
</div>

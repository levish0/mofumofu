<!-- layout.svelte -->
<script lang="ts">
    import { authStore } from '$lib/stores/auth.context.svelte';


  import { onMount } from 'svelte';

  let { children } = $props();

  // 브라우저 환경에서 초기화 보장
  onMount(() => {
    authStore.ensureInitialized();
  });
</script>

{@render children()}
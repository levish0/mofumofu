<!-- layout.svelte -->
<script lang="ts">
  import { createAuthContext } from '$lib/stores/auth.context.svelte';
  import { onMount } from 'svelte';

  let { children } = $props();

  // onMount에서 Context 생성
  onMount(() => {
    const authStore = createAuthContext();
    authStore.init();
  });
</script>

{@render children()}

<script lang="ts">
  import { onMount, setContext } from 'svelte';
  import '../app.css';
  import { authStore } from '$lib/stores/auth.svelte';

  onMount(() => {
    authStore.init();
    setContext('auth', authStore);
  });

  let { children } = $props();
</script>

{@render children()}

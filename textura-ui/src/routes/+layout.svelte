<script lang="ts">
  import { authStore } from '$lib/stores/auth.context.svelte';
  import '../app.css';

  import { onMount } from 'svelte';

  let { children } = $props();


  onMount(() => {
    authStore.ensureInitialized();
  });
</script>

{@render children()}

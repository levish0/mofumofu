<script lang="ts">
  import { setContext } from 'svelte';
  import '../app.css';
  import { authStore } from '$lib/stores/auth.svelte';

  setContext('auth', authStore);
  let { children } = $props();
</script>

{@render children()}

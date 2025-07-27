<script lang="ts">
  import '../app.css';
  import { ModeWatcher } from 'mode-watcher';

  let { children } = $props();
</script>

<ModeWatcher defaultMode="dark" />
<div class="font-pretendard">
  {@render children()}
</div>

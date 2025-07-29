<script lang="ts">
  export let h = 9; 
  export let w = 9; 

  const hClass = `h-${h}`;
  const wClass = `w-${w}`;
</script>

<div class={`animate-pulse rounded-full bg-white/10 ${hClass} ${wClass}`}></div>

<script lang="ts">
	import { userStore } from '$lib/stores/user.svelte';
	import EmailVerificationInput from './EmailVerificationInput.svelte';

	const user = $derived(userStore.user);
</script>

<div class="dark:text-mofu-dark-200 text-mofu-light-200 min-h-screen">
	<div class="space-y-4">
		{#if user?.email}
			<EmailVerificationInput email={user.email} />
		{/if}
	</div>
</div>

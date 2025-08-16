<script lang="ts">
	import { ExclamationTriangle, Icon } from 'svelte-hero-icons';
	import { Button } from '$lib/components/ui/button';
	import { goto } from '$app/navigation';

	let { 
		isVisible, 
		title = "로그인이 필요한 서비스입니다",
		description = "이 기능을 이용하려면 로그인해 주세요.",
		showHomeButton = true,
		showLoginButton = true
	} = $props<{
		isVisible: boolean;
		title?: string;
		description?: string;
		showHomeButton?: boolean;
		showLoginButton?: boolean;
	}>();
</script>

{#if isVisible}
	<div class="text-mofu-dark-200 bg-mofu-dark-900 flex min-h-screen items-center justify-center">
		<div class="w-full max-w-md space-y-8 p-8">
			<div class="text-center">
				<div class="space-y-4">
					<div class="text-rose-600">
						<Icon src={ExclamationTriangle} solid size="40" class="inline-block" />
					</div>
					<h2 class="text-xl font-semibold">{title}</h2>
					<p class="text-sm text-gray-400">{description}</p>
					<div class="flex flex-col space-y-3 pt-4">
						{#if showLoginButton}
							<Button onclick={() => goto('/account/signin')} class="w-full">
								로그인하기
							</Button>
						{/if}
						{#if showHomeButton}
							<Button
								onclick={() => goto('/')}
								variant="ghost"
								class="w-full dark:text-mofu-dark-300 rounded-md text-sm hover:opacity-70"
							>
								홈으로 가기
							</Button>
						{/if}
					</div>
				</div>
			</div>
		</div>
	</div>
{/if}
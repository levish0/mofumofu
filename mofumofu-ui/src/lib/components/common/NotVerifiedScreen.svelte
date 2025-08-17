<script lang="ts">
	import { ShieldExclamation, Icon } from 'svelte-hero-icons';
	import { Button } from '$lib/components/ui/button';
	import { goto } from '$app/navigation';

	let {
		isVisible,
		title = '이메일 인증이 필요한 서비스입니다',
		description = '이 기능을 이용하려면 이메일 인증을 완료해 주세요.',
		showHomeButton = true,
		showSettingsButton = true
	} = $props<{
		isVisible: boolean;
		title?: string;
		description?: string;
		showHomeButton?: boolean;
		showSettingsButton?: boolean;
	}>();
</script>

{#if isVisible}
	<div class="text-mofu-dark-200 bg-mofu-dark-900 flex min-h-screen items-center justify-center">
		<div class="w-full max-w-md space-y-8 p-8">
			<div class="text-center">
				<div class="space-y-4">
					<div class="text-amber-600">
						<Icon src={ShieldExclamation} solid size="40" class="inline-block" />
					</div>
					<h2 class="text-xl font-semibold">{title}</h2>
					<p class="text-sm text-gray-400">{description}</p>
					<div class="flex flex-col space-y-3 pt-4">
						{#if showSettingsButton}
							<Button onclick={() => goto('/settings#account')} class="w-full">계정 설정으로 이동</Button>
						{/if}
						{#if showHomeButton}
							<Button
								onclick={() => goto('/')}
								variant="ghost"
								class="dark:text-mofu-dark-300 w-full rounded-md text-sm hover:opacity-70"
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
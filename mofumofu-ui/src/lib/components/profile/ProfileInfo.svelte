<script lang="ts">
	import type { UserInfoResponse } from '$lib/api/user/types';
	import { Briefcase, CalendarDays, Icon, MapPin } from 'svelte-hero-icons';
	import * as m from '../../../paraglide/messages';

	type Props = {
		profile: UserInfoResponse;
	};

	const { profile }: Props = $props();
</script>

<!-- Profile Content -->
<div class="px-2 pt-14 pb-4">
	<!-- Profile Info -->
	<div class="space-y-2">
		<div>
			<h1 class="text-xl font-bold text-gray-900 dark:text-white">
				{profile.name}
			</h1>
			<p class="dark:text-mofu-dark-300 text-sm">
				@{profile.handle}
			</p>
		</div>

		<!-- Bio -->
		{#if profile.bio}
			<p class="dark:text-mofu-dark-200 text-md text-gray-700">
				{profile.bio}
			</p>
		{/if}

		<!-- Location and Join Date -->
		<div class="dark:text-mofu-dark-300 flex items-center gap-4 text-sm">
			<div class="flex items-center gap-1">
				<Icon src={MapPin} class="h-4 w-4" />
				<span>tokyo</span>
			</div>
			<div class="flex items-center gap-1">
				<Icon src={Briefcase} class="h-4 w-4" />
				<span>mofumofu</span>
			</div>
			<div class="flex items-center gap-1">
				<Icon src={CalendarDays} class="h-4 w-4" />
				<span>Joined August 2017</span>
			</div>
		</div>

		<!-- Stats -->
		<div class="flex items-center space-x-4 text-sm">
			<div>
				<span class="font-bold text-gray-900 dark:text-white">427</span>
				<span class="dark:text-mofu-dark-300">{m.profile_following()}</span>
			</div>
			<div>
				<span class="font-bold text-gray-900 dark:text-white">152.1K</span>
				<span class="dark:text-mofu-dark-300">{m.profile_followers()}</span>
			</div>
		</div>
	</div>
</div>

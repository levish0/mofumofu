<script lang="ts">
	type Props = {
		profile: {
			name: string;
			handle: string;
		};
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
			<p class="text-sm text-gray-500 dark:text-gray-400">
				@{profile.handle}
			</p>
		</div>

		<!-- Bio -->
		<p class="text-sm text-gray-700 dark:text-gray-300">
			✨ 아 파이썬으로 백엔드 짤걸<br />
			✨ 괜히 러스트로 짜가지고 <br />
			⚠️ 아 자고 싶다
		</p>

		<!-- Location and Join Date -->
		<div class="space-y-1 text-sm text-gray-500 dark:text-gray-400">
			<div class="flex items-center gap-1">
				<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"
					></path>
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"
					></path>
				</svg>
				<span>tokyo</span>
			</div>
			<div class="flex items-center gap-1">
				<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M8 7V3a2 2 0 012-2h4a2 2 0 012 2v4m-6 4v10m6-10v10M6 21h12a2 2 0 002-2V9a2 2 0 00-2-2H6a2 2 0 00-2 2v10a2 2 0 002 2z"
					></path>
				</svg>
				<span>Joined August 2017</span>
			</div>
		</div>

		<!-- Stats -->
		<div class="flex items-center space-x-4 text-sm">
			<div>
				<span class="font-bold text-gray-900 dark:text-white">427</span>
				<span class="text-gray-500 dark:text-gray-400">Following</span>
			</div>
			<div>
				<span class="font-bold text-gray-900 dark:text-white">152.1K</span>
				<span class="text-gray-500 dark:text-gray-400">Followers</span>
			</div>
		</div>

		<!-- Additional Info -->
		<p class="text-xs text-gray-500 dark:text-gray-400">Not followed by anyone you're following</p>
	</div>
</div>

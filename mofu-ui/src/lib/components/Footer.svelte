<!-- src/lib/components/Footer.svelte -->

<footer class="bg-mofu-dark-900 relative pt-20 pb-20">
	<div class="custom-width container mx-auto max-w-7xl px-8 py-8">
		<div class="grid grid-cols-1 gap-6 sm:grid-cols-3">
			<!-- About Section -->
			<div class="space-y-2">
				<h3 class="text-xl font-semibold text-zinc-900 dark:text-zinc-100">もふもふ。</h3>
				<p class="text-xs text-zinc-600 dark:text-zinc-400">
					<a href="/" class="text-zinc-700 hover:underline dark:text-zinc-300">By Levish</a>
				</p>
			</div>

			<!-- Legal -->
			<div class="space-y-2">
				<h3 class="text-md font-semibold text-zinc-900 dark:text-zinc-100">Legal</h3>
				<ul class="space-y-1 text-sm">
					<li>
						<a href="/privacy" class="text-zinc-600 hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-zinc-100">
							Privacy Policy
						</a>
					</li>
					<li>
						<a href="/terms" class="text-zinc-600 hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-zinc-100">
							Terms of Service
						</a>
					</li>
				</ul>
			</div>
		</div>
	</div>
</footer>

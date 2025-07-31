<script lang="ts">
	let selectedCountry = $state('Germany — Deutsch...');
	let showCountryOnProfile = $state(true);
	let nativeName = $state('');
	let englishName = $state('');
	let showNameOnProfile = $state(true);
	let birthday = $state('');
	let showBirthYearOnProfile = $state(true);
	let showBirthdayOnProfile = $state(true);
	let selectedGender = $state('');

	const countries = [
		'Germany — Deutsch...',
		'United States — English',
		'South Korea — 한국어',
		'Japan — 日本語',
		'France — Français',
		'United Kingdom — English'
	];

	const genders = ['Male', 'Female', 'Non-binary', 'Prefer not to say'];
</script>

<div>
	<h1 class="mb-2 text-3xl font-bold">Personal Info</h1>
	<p class="text-muted-foreground mb-8">Manage your personal information and preferences.</p>

	<div class="space-y-8">
		<!-- Country/Region Section -->
		<div class="border-mofu-dark-700 space-y-4  border-b pb-8">
			<div>
				<h3 class="mb-1 text-lg font-medium">Country/Region</h3>
				<p class="text-muted-foreground mb-4 text-sm">You can optionally display this on your profile.</p>
			</div>

			<div class="space-y-4">
				<div>
					<label class="mb-2 block text-sm font-medium">Country/Region</label>
					<select
						class="bg-background border-mofu-dark-700 focus:ring-primary w-full rounded-lg border px-3 py-2 focus:ring-2 focus:outline-none"
						bind:value={selectedCountry}
					>
						{#each countries as country}
							<option value={country}>{country}</option>
						{/each}
					</select>
				</div>

				<div class="flex items-center justify-between">
					<span class="text-sm font-medium">Show country/region on profile</span>
					<button
						class="relative inline-flex h-6 w-11 items-center rounded-full bg-mofu-dark-800 transition-colors {showCountryOnProfile
							? 'bg-primary'
							: 'bg-muted'}"
						onclick={() => (showCountryOnProfile = !showCountryOnProfile)}
						aria-label="Toggle show country/region on profile"
					>
						<span
							class="inline-block h-4 w-4 transform rounded-full bg-mofu-dark-200 transition-transform {showCountryOnProfile
								? 'translate-x-6'
								: 'translate-x-1'}"
						></span>
					</button>
				</div>
			</div>
		</div>

		<!-- Name Section -->
		<div class="border-mofu-dark-700 space-y-4 border-b pb-8">
			<div>
				<h3 class="mb-1 text-lg font-medium">Name</h3>
				<p class="text-muted-foreground mb-4 text-sm">You can optionally display this on your profile.</p>
			</div>

			<div class="grid grid-cols-2 gap-4">
				<div>
					<label class="text-muted-foreground mb-2 block text-sm">in native language</label>
					<input
						type="text"
						class="bg-background border-mofu-dark-700 focus:ring-primary w-full rounded-lg border px-3 py-2 focus:ring-2 focus:outline-none"
						bind:value={nativeName}
					/>
				</div>
				<div>
					<label class="text-muted-foreground mb-2 block text-sm">in English</label>
					<input
						type="text"
						class="bg-background border-mofu-dark-700 focus:ring-primary w-full rounded-lg border px-3 py-2 focus:ring-2 focus:outline-none"
						bind:value={englishName}
					/>
				</div>
			</div>

			<div class="flex items-center justify-between">
				<span class="text-sm font-medium">Show name on profile</span>
				<button
					class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors {showNameOnProfile
						? 'bg-primary'
						: 'bg-muted'}"
					onclick={() => (showNameOnProfile = !showNameOnProfile)}
				>
					<span
						class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform {showNameOnProfile
							? 'translate-x-6'
							: 'translate-x-1'}"
					></span>
				</button>
			</div>
		</div>

		<!-- Birthday Section -->
		<div class="border-mofu-dark-700 space-y-4 border-b pb-8">
			<div>
				<h3 class="mb-1 text-lg font-medium">Birthday</h3>
				<p class="text-muted-foreground mb-4 text-sm">You can optionally display this on your profile.</p>
			</div>

			<div>
				<input
					type="date"
					class="bg-background border-mofu-dark-700 focus:ring-primary w-full rounded-lg border px-3 py-2 focus:ring-2 focus:outline-none"
					bind:value={birthday}
					placeholder="mm / dd / yyyy"
				/>
			</div>

			<div class="space-y-3">
				<div class="flex items-center justify-between">
					<span class="text-sm font-medium">Show birth year on profile</span>
					<button
						class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors {showBirthYearOnProfile
							? 'bg-primary'
							: 'bg-muted'}"
						onclick={() => (showBirthYearOnProfile = !showBirthYearOnProfile)}
					>
						<span
							class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform {showBirthYearOnProfile
								? 'translate-x-6'
								: 'translate-x-1'}"
						></span>
					</button>
				</div>

				<div class="flex items-center justify-between">
					<span class="text-sm font-medium">Show birthday on profile</span>
					<button
						class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors {showBirthdayOnProfile
							? 'bg-primary'
							: 'bg-muted'}"
						onclick={() => (showBirthdayOnProfile = !showBirthdayOnProfile)}
					>
						<span
							class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform {showBirthdayOnProfile
								? 'translate-x-6'
								: 'translate-x-1'}"
						></span>
					</button>
				</div>
			</div>
		</div>

		<!-- Gender Section -->
		<div class="space-y-4">
			<div>
				<h3 class="mb-1 text-lg font-medium">Gender</h3>
			</div>

			<div class="space-y-2">
				{#each genders as gender}
					<label class="flex cursor-pointer items-center gap-3">
						<input
							type="radio"
							name="gender"
							value={gender}
							bind:group={selectedGender}
							class="text-primary bg-background border-mofu-dark-700 focus:ring-primary h-4 w-4"
						/>
						<span class="text-sm">{gender}</span>
					</label>
				{/each}
			</div>
		</div>
	</div>
</div>

<script lang="ts">
	import { Switch } from '../ui/switch';
	import { Input } from '../ui/input';
	import { Button } from '../ui/button';
	import { Calendar } from '../ui/calendar';
	import * as Popover from '../ui/popover';
	import { CalendarDays, Icon } from 'svelte-hero-icons';
	import { format } from 'date-fns';
	import { cn } from '$lib/utils';
	import { CalendarDate } from '@internationalized/date';

	let selectedCountry = $state('germany');
	let showCountryOnProfile = $state(true);
	let nativeName = $state('');
	let englishName = $state('');
	let showNameOnProfile = $state(true);
	import type { DateValue } from '@internationalized/date';
	let birthday = $state<DateValue[] | undefined>(undefined);
	let showBirthYearOnProfile = $state(true);
	let showBirthdayOnProfile = $state(true);
	let selectedGender = $state('unspecified');
	let pronouns = $state('they/them');
	let showPronounsOnProfile = $state(true);

	const countries = [
		{ value: 'germany', label: 'Germany — Deutsch...', flag: '🇩🇪' },
		{ value: 'usa', label: 'United States — English', flag: '🇺🇸' },
		{ value: 'south-korea', label: 'South Korea — 한국어', flag: '🇰🇷' },
		{ value: 'japan', label: 'Japan — 日本語', flag: '🇯🇵' },
		{ value: 'france', label: 'France — Français', flag: '🇫🇷' },
		{ value: 'uk', label: 'United Kingdom — English', flag: '🇬🇧' }
	];

	const genders = [
		{ value: 'unspecified', label: 'Unspecified' },
		{ value: 'male', label: 'Male' },
		{ value: 'female', label: 'Female' },
		{ value: 'non-binary', label: 'Non-binary' }
	];

	const selectedCountryData = $derived(countries.find((c) => c.value === selectedCountry) || countries[0]);
	const selectedGenderData = $derived(genders.find((g) => g.value === selectedGender) || genders[0]);
</script>

<div class="min-h-screen p-6 text-gray-100 md:p-8 lg:p-10">
	<div class="mx-auto max-w-3xl space-y-8">
		<div class=" space-y-4 border-slate-700">
			<h2 class="text-xl font-semibold">Name</h2>
			<p class="text-sm text-gray-400">You can optionally display this on your profile.</p>
			<div class="space-y-4 pt-4">
				<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
					<div>
						<Input
							id="name-native"
							placeholder=""
							class="w-full border-slate-700 bg-slate-800 text-gray-100"
							bind:value={nativeName}
						/>
						<label for="name-native" class="mt-1 block text-xs text-gray-400"> in native language </label>
					</div>
					<div>
						<Input
							id="name-english"
							placeholder=""
							class="w-full border-slate-700 bg-slate-800 text-gray-100"
							bind:value={englishName}
						/>
						<label for="name-english" class="mt-1 block text-xs text-gray-400"> in English </label>
					</div>
				</div>
				<div class="flex items-center justify-between">
					<label for="show-name">Show name on profile</label>
					<Switch
						id="show-name"
						bind:checked={showNameOnProfile}
						class="data-[state=checked]:bg-blue-500 data-[state=unchecked]:bg-gray-600"
					/>
				</div>
			</div>
		</div>

		<div class="space-y-4 border-t border-slate-700 pt-8">
			<h2 class="text-xl font-semibold">Birthday</h2>
			<p class="text-sm text-gray-400">You can optionally display this on your profile.</p>
			<div class="space-y-4 pt-4">
				<Popover.Root>
					<Popover.Trigger
						class={cn(
							'inline-flex w-full items-center justify-start rounded-md border border-slate-700 bg-slate-800 px-3 py-2 text-left text-sm font-normal text-gray-100 outline-none hover:bg-slate-700 focus:ring-2 focus:ring-blue-500',
							!birthday && 'text-gray-400'
						)}
					>
						<Icon src={CalendarDays} class="mr-2 h-4 w-4" />
						{birthday && birthday.length > 0 ? format(birthday[0].toDate('UTC'), 'PPP') : 'mm / dd / yyyy'}
					</Popover.Trigger>
					<Popover.Content class="w-auto border-slate-700 bg-slate-800 p-0 text-gray-100">
						<Calendar type="multiple" bind:value={birthday} class="text-gray-100" />
					</Popover.Content>
				</Popover.Root>
				<div class="flex items-center justify-between">
					<label for="show-birth-year">Show birth year on profile</label>
					<Switch
						id="show-birth-year"
						bind:checked={showBirthYearOnProfile}
						class="data-[state=checked]:bg-blue-500 data-[state=unchecked]:bg-gray-600"
					/>
				</div>
				<div class="flex items-center justify-between">
					<label for="show-birthday">Show birthday on profile</label>
					<Switch
						id="show-birthday"
						bind:checked={showBirthdayOnProfile}
						class="data-[state=checked]:bg-blue-500 data-[state=unchecked]:bg-gray-600"
					/>
				</div>
			</div>
		</div>

		<div class="space-y-4 border-t border-slate-700 pt-8">
			<h2 class="text-xl font-semibold">Gender</h2>
			<p class="text-sm text-gray-400">You can optionally display this on your profile.</p>
			<div class="space-y-4 pt-4">
				<div>
					<label for="gender" class="text-gray-300">Gender</label>
					<select
						id="gender"
						class="mt-2 w-full rounded-md border border-slate-700 bg-slate-800 px-3 py-2 text-sm text-gray-100 outline-none focus:ring-2 focus:ring-blue-500 md:ml-auto md:w-auto"
						bind:value={selectedGender}
					>
						{#each genders as gender}
							<option value={gender.value}>{gender.label}</option>
						{/each}
					</select>
				</div>
			</div>
		</div>

		<div class="space-y-4 border-t border-slate-700 pt-8">
			<div class="space-y-4 pt-4">
				<div class="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
					<label for="pronouns" class="text-gray-300 md:w-1/3">Pronouns</label>
					<Input
						id="pronouns"
						bind:value={pronouns}
						class="w-full border-slate-700 bg-slate-800 text-gray-100 md:w-2/3"
					/>
				</div>
				<div class="flex items-center justify-between">
					<label for="show-pronouns">Show pronouns on profile</label>
					<Switch
						id="show-pronouns"
						bind:checked={showPronounsOnProfile}
						class="data-[state=checked]:bg-blue-500 data-[state=unchecked]:bg-gray-600"
					/>
				</div>
			</div>
		</div>
	</div>
</div>

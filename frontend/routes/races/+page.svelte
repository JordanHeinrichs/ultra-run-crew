<script lang="ts">
	import { resolve } from '$app/paths';
	import type { PageData } from './$types';
	import PlusIcon from '@lucide/svelte/icons/plus';
	import SearchIcon from '@lucide/svelte/icons/search';

	let searchQuery = $state('');

	let { data }: { data: PageData } = $props();

	let filteredRaces = $derived(
		data.races.filter(
			(race) =>
				race.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
				race.runner_name.toLowerCase().includes(searchQuery.toLowerCase())
		)
	);
</script>

<div class="mb-6 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
	<div>
		<h1 class="text-2xl font-black tracking-tight text-primary uppercase italic sm:text-3xl">
			Race Dashboards
		</h1>
	</div>

	<a href={resolve('/races/new')} class="btn gap-2 uppercase btn-primary">
		<PlusIcon size="20"></PlusIcon>
		Add Race
	</a>
</div>

<div class="form-control mb-6 w-full max-w-md">
	<div class="relative">
		<input
			type="text"
			placeholder="Search race or runner..."
			bind:value={searchQuery}
			class="input-bordered input w-full pr-10"
		/>
		<SearchIcon class="absolute top-3 right-3 h-5 w-5 text-base-content/40" size="20"></SearchIcon>
	</div>
</div>

<div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
	{#each filteredRaces as race (race.id)}
		<div class="card border border-base-300 bg-base-100 shadow-md transition-all hover:shadow-lg">
			<div class="card-body p-5">
				<div class="flex items-start justify-between gap-2">
					<div>
						<h2 class="card-title text-lg font-bold">{race.name}</h2>
						<p class="text-xs text-base-content/60">{race.event_date}</p>
					</div>
				</div>

				<div class="divider my-2"></div>

				<div class="space-y-1.5 text-xs">
					<div class="flex justify-between">
						<span class="text-base-content/70">Runner:</span>
						<span class="font-semibold">{race.runner_name}</span>
					</div>
				</div>

				<div class="mt-4 card-actions">
					<a href={resolve(`/races/${race.id}`)} class="btn btn-block uppercase btn-primary btn-sm">
						Enter Dashboard
					</a>
				</div>
			</div>
		</div>
	{/each}
</div>

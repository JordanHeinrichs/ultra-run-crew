<script lang="ts">
	import { resolve } from '$app/paths';
	import type { PageData } from './$types';
	import SettingsIcon from '@lucide/svelte/icons/settings';
	import CalendarIcon from '@lucide/svelte/icons/calendar';
	import UserIcon from '@lucide/svelte/icons/user';

	let { data }: { data: PageData } = $props();

	function formatDate(dateString: string) {
		return new Date(dateString).toLocaleDateString('en-CA', {
			weekday: 'short',
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	}
</script>

<div class="mx-auto max-w-6xl space-y-8 px-4 py-8">
	<div class="flex flex-col justify-between gap-4 sm:flex-row sm:items-center">
		<div>
			<h1 class="text-4xl font-extrabold text-base-content">{data.race.name}</h1>
			<p class="mt-1 text-sm text-base-content/60">
				Race • Created {new Date(data.race.createdAt).toLocaleDateString('en-CA')}
			</p>
		</div>

		<a href={resolve(`/races/${data.race.id}/config`)} class="btn btn-primary">
			<SettingsIcon size="20"></SettingsIcon>
			Configure Race
		</a>
	</div>

	<div
		class="stats w-full stats-vertical border border-base-200 bg-base-100 shadow-lg sm:stats-horizontal"
	>
		<div class="stat">
			<div class="stat-figure text-primary">
				<CalendarIcon size="30"></CalendarIcon>
			</div>
			<div class="stat-title">Event Date</div>
			<div class="stat-value text-2xl">{formatDate(data.race.eventDate)}</div>
		</div>

		<div class="stat">
			<div class="stat-figure text-secondary">
				<UserIcon size="30"></UserIcon>
			</div>
			<div class="stat-title">Runner Profile</div>
			<!-- TODO: Link to runner pages -->
			<a href={resolve(`/`)} class="stat-value text-2xl">{data.runner.name}</a>
		</div>
	</div>

	<div class="grid grid-cols-1 gap-8 lg:grid-cols-2">
		<div class="card border border-base-200 bg-base-100 shadow-xl lg:col-span-2">
			<div class="card-body">
				<div class="mb-4 flex items-center justify-between">
					<h2 class="card-title text-xl">Route Information & Pacing</h2>
					<button class="btn btn-outline btn-sm">Edit Route</button>
				</div>
				<div class="overflow-x-auto">
					<table class="table w-full table-zebra">
						<thead>
							<tr>
								<th>Segment</th>
								<th>Distance</th>
								<th>Elevation Gain</th>
								<th>Target Pace</th>
							</tr>
						</thead>
						<tbody>
							<tr>
								<td>Start to AS1</td>
								<td>10.5 km</td>
								<td>+450m</td>
								<td>6:30 /km</td>
							</tr>
							<tr>
								<td>AS1 to Ridge</td>
								<td>8.2 km</td>
								<td>+800m</td>
								<td>8:15 /km</td>
							</tr>
						</tbody>
					</table>
				</div>
			</div>
		</div>

		<div class="card border border-base-200 bg-base-100 shadow-xl">
			<div class="card-body">
				<div class="mb-4 flex items-center justify-between">
					<h2 class="card-title text-xl">Aid Stations</h2>
					<button class="btn btn-outline btn-sm">Add Station</button>
				</div>
				<div class="overflow-x-auto">
					<table class="table w-full">
						<thead>
							<tr>
								<th>Location</th>
								<th>Crew Access</th>
								<th></th>
							</tr>
						</thead>
						<tbody>
							<tr>
								<td>
									<div class="font-bold">AS1: Whispering Pines</div>
									<div class="text-sm opacity-50">km 10.5</div>
								</td>
								<td>
									<div class="badge badge-sm badge-success">Crew Allowed</div>
								</td>
								<th class="text-right">
									<a
										href={resolve(`/races/${data.race.id}/aid-stations/1`)}
										class="btn btn-ghost btn-xs">Configure</a
									>
								</th>
							</tr>
							<tr>
								<td>
									<div class="font-bold">AS2: High Ridge</div>
									<div class="text-sm opacity-50">km 18.7</div>
								</td>
								<td>
									<div class="badge badge-sm badge-error">No Crew</div>
								</td>
								<th class="text-right">
									<a
										href={resolve(`/races/${data.race.id}/aid-stations/2`)}
										class="btn btn-ghost btn-xs">Configure</a
									>
								</th>
							</tr>
						</tbody>
					</table>
				</div>
			</div>
		</div>

		<div class="card border border-base-200 bg-base-100 shadow-xl">
			<div class="card-body">
				<div class="mb-4 flex items-center justify-between">
					<h2 class="card-title text-xl">Nutrition Plan</h2>
					<button class="btn btn-outline btn-sm">Edit Plan</button>
				</div>
				<div class="overflow-x-auto">
					<table class="table w-full">
						<thead>
							<tr>
								<th>Frequency</th>
								<th>Intake Strategy</th>
							</tr>
						</thead>
						<tbody>
							<tr>
								<td class="font-semibold text-nowrap">Every 45 mins</td>
								<td>1x Energy Gel (25g Carbs)</td>
							</tr>
							<tr>
								<td class="font-semibold text-nowrap">Every Hour</td>
								<td>500ml Electrolyte mix + 1 Salt tab</td>
							</tr>
							<tr>
								<td class="font-semibold text-nowrap">AS1 (km 10.5)</td>
								<td>Refill flasks, grab solid food (banana/bar)</td>
							</tr>
						</tbody>
					</table>
				</div>
			</div>
		</div>
	</div>
</div>

<script lang="ts">
	import { resolve } from '$app/paths';
	import type { PageData } from './$types';

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
			<svg
				xmlns="http://www.w3.org/2000/svg"
				class="mr-1 h-5 w-5"
				fill="none"
				viewBox="0 0 24 24"
				stroke="currentColor"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
				/>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
				/>
			</svg>
			Configure Race
		</a>
	</div>

	<div
		class="stats w-full stats-vertical border border-base-200 bg-base-100 shadow-lg sm:stats-horizontal"
	>
		<div class="stat">
			<div class="stat-figure text-primary">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="h-8 w-8"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
					/>
				</svg>
			</div>
			<div class="stat-title">Event Date</div>
			<div class="stat-value text-2xl">{formatDate(data.race.eventDate)}</div>
		</div>

		<div class="stat">
			<div class="stat-figure text-secondary">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="h-8 w-8"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
					/>
				</svg>
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

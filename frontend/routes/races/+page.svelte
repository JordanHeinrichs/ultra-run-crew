<script lang="ts">
  import { resolve } from '$app/paths';
	import type { PageData } from './$types';

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

  <a href={resolve('/races/new')} class="btn btn-primary gap-2 uppercase">
    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
    </svg>
    Add Race
  </a>
</div>

<div class="form-control mb-6 w-full max-w-md">
  <div class="relative">
    <input
      type="text"
      placeholder="Search race or runner..."
      bind:value={searchQuery}
      class="input input-bordered w-full pr-10"
    />
    <svg
      xmlns="http://www.w3.org/2000/svg"
      class="absolute right-3 top-3 h-5 w-5 text-base-content/40"
      fill="none"
      viewBox="0 0 24 24"
      stroke="currentColor"
    >
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
    </svg>
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
            <span class="font-semibold">{race.runner}</span>
          </div>
        </div>

        <div class="card-actions mt-4">
          <a href={resolve(`/races/${race.id}`)} class="btn btn-primary btn-block btn-sm uppercase">
            Enter Dashboard
          </a>
        </div>
      </div>
    </div>
  {/each}
</div>

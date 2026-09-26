<script lang="ts">
	import { resolve } from '$app/paths';
	import WaypointsIcon from '@lucide/svelte/icons/waypoints';
	import ArrowRightIcon from '@lucide/svelte/icons/arrow-right';
</script>

<div class="flex min-h-screen flex-col justify-between bg-base-200 p-6 font-sans text-base-content">
	<main class="mx-auto my-auto flex w-full max-w-md flex-col items-center py-8 text-center">
		<div
			class="mb-6 flex h-20 w-20 items-center justify-center rounded-2xl bg-primary text-primary-content shadow-lg shadow-primary/20"
		>
			<WaypointsIcon size="30"></WaypointsIcon>
		</div>

		<h1 class="text-4xl font-black tracking-tight text-primary uppercase italic sm:text-5xl">
			Ultra<span class="text-secondary">Crew</span>
		</h1>

		<p class="mt-3 max-w-xs text-base leading-relaxed font-medium text-base-content/70 sm:text-lg">
			Dynamic ETAs and zero-friction command center for ultra-marathon crews.
		</p>

		<div class="mt-10 grid w-full grid-cols-3 gap-3 text-xs font-bold">
			<div
				class="flex flex-col items-center gap-1.5 rounded-xl border border-base-300/60 bg-base-100 p-3 shadow-sm"
			>
				<span class="text-lg">⚡</span>
				<span>Auto-ETAs</span>
			</div>
			<div
				class="flex flex-col items-center gap-1.5 rounded-xl border border-base-300/60 bg-base-100 p-3 shadow-sm"
			>
				<span class="text-lg">📡</span>
				<span>Offline Support</span>
			</div>
			<div
				class="flex flex-col items-center gap-1.5 rounded-xl border border-base-300/60 bg-base-100 p-3 shadow-sm"
			>
				<span class="text-lg">👆</span>
				<span>Easy to Use</span>
			</div>
		</div>
	</main>

	<footer class="mx-auto w-full max-w-md space-y-3 pb-4">
		<a
			href={resolve('/races')}
			class="btn btn-block gap-2 uppercase shadow-lg shadow-primary/20 btn-lg btn-primary"
		>
			<span>Get Started</span>
			<ArrowRightIcon></ArrowRightIcon>
		</a>

		<p class="text-center font-mono text-xs text-base-content/60">
			No App Store required • Works completely offline
		</p>
	</footer>
</div>

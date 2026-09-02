<script>
	import { resolve } from '$app/paths';

	// Root Splash Page for UltraCrew
</script>

<div
	class="flex min-h-screen flex-col justify-between bg-slate-950 p-6 font-sans text-slate-100 select-none"
>
	<header class="mx-auto flex w-full max-w-md items-center justify-between pt-2">
		<div
			class="flex items-center gap-2 rounded-full border border-slate-800 bg-slate-900 px-3 py-1.5"
		>
			<span class="inline-block h-2.5 w-2.5 animate-pulse rounded-full bg-emerald-500"></span>
			<span class="font-mono text-xs tracking-wider text-slate-300 uppercase"
				>Offline Engine Ready</span
			>
		</div>
		<span class="font-mono text-xs text-slate-500">v0.1.0</span>
	</header>

	<main class="mx-auto my-auto flex w-full max-w-md flex-col items-center py-8 text-center">
		<div
			class="mb-6 rounded-2xl border border-amber-500/30 bg-amber-500/10 p-4 text-amber-400 shadow-lg shadow-amber-500/5"
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				class="h-12 w-12"
				fill="none"
				viewBox="0 0 24 24"
				stroke="currentColor"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M13 10V3L4 14h7v7l9-11h-7z"
				/>
			</svg>
		</div>

		<h1 class="text-4xl font-black tracking-tight text-white uppercase italic sm:text-5xl">
			Ultra<span class="text-amber-500">Crew</span>
		</h1>

		<p class="mt-3 max-w-xs text-base leading-relaxed font-medium text-slate-400 sm:text-lg">
			Dynamic ETAs and zero-friction command center for ultra-marathon crews.
		</p>

		<div class="mt-10 grid w-full grid-cols-3 gap-2 text-xs font-bold text-slate-300">
			<div
				class="flex flex-col items-center gap-1.5 rounded-xl border border-slate-800 bg-slate-900/90 p-3"
			>
				<span class="text-base text-amber-400">⚡</span>
				<span>Auto-ETAs</span>
			</div>
			<div
				class="flex flex-col items-center gap-1.5 rounded-xl border border-slate-800 bg-slate-900/90 p-3"
			>
				<span class="text-base text-emerald-400">📡</span>
				<span>Zero Signal</span>
			</div>
			<div
				class="flex flex-col items-center gap-1.5 rounded-xl border border-slate-800 bg-slate-900/90 p-3"
			>
				<span class="text-base text-sky-400">👆</span>
				<span>1-Tap Sync</span>
			</div>
		</div>
	</main>

	<footer class="mx-auto w-full max-w-md space-y-3 pb-4">
		<a
			href={resolve('/login')}
			class="flex h-16 w-full cursor-pointer items-center justify-center gap-3 rounded-2xl border-b-4 border-amber-600 bg-amber-500 text-center text-xl font-extrabold tracking-wider text-slate-950 uppercase shadow-lg shadow-amber-500/10 transition-all hover:bg-amber-400 active:scale-[0.98] active:bg-amber-600"
		>
			<span>Get Started</span>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				class="h-6 w-6"
				fill="none"
				viewBox="0 0 24 24"
				stroke="currentColor"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="3"
					d="M14 5l7 7m0 0l-7 7m7-7H3"
				/>
			</svg>
		</a>

		<p class="text-center font-mono text-xs text-slate-500">
			No App Store required • Works completely offline
		</p>
	</footer>
</div>

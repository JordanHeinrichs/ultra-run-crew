<script>
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';

	let name = '';
	let email = '';
	let password = '';
	let errorMessage = '';
	let loading = false;

	async function handleRegister() {
		errorMessage = '';
		loading = true;

		try {
			const response = await fetch('/api/auth/register', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({ name, email, password })
			});

			if (response.status === 200) {
				goto(resolve('/races'));
			} else {
				errorMessage = 'Registration failed. Please check your information.';
			}
		} catch (err) {
			console.error(err);
			errorMessage = 'Registration failed. Network or server connection error.';
		} finally {
			loading = false;
		}
	}
</script>

<div
	class="flex min-h-screen flex-col justify-between bg-slate-950 p-6 font-sans text-slate-100 select-none"
>
	<!-- Top Navigation Bar -->
	<header class="mx-auto flex w-full max-w-md items-center justify-between pt-2">
		<a
			href={resolve('/')}
			class="flex items-center gap-1 text-sm font-semibold text-slate-400 hover:text-white"
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				class="h-5 w-5"
				fill="none"
				viewBox="0 0 24 24"
				stroke="currentColor"
			>
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
			</svg>
			<span>Back</span>
		</a>
		<span class="font-mono text-xs font-semibold tracking-widest text-amber-500 uppercase"
			>New Account</span
		>
	</header>

	<!-- Register Form Container -->
	<main class="mx-auto my-auto w-full max-w-md py-6">
		<div class="mb-8">
			<h1 class="text-3xl font-black tracking-tight text-white uppercase italic">
				Create <span class="text-amber-500">Account</span>
			</h1>
			<p class="mt-1 text-sm text-slate-400">Sign up to manage and sync crew operations offline.</p>
		</div>

		<!-- Error Alert -->
		{#if errorMessage}
			<div
				class="mb-6 flex items-center gap-3 rounded-xl border border-rose-500/40 bg-rose-500/10 p-4 text-sm font-semibold text-rose-400"
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="h-5 w-5 flex-shrink-0"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
					/>
				</svg>
				<span>{errorMessage}</span>
			</div>
		{/if}

		<form on:submit|preventDefault={handleRegister} class="space-y-5">
			<!-- Full Name Input -->
			<div>
				<label
					for="name"
					class="mb-2 block font-mono text-xs font-bold tracking-wider text-slate-300 uppercase"
				>
					Full Name
				</label>
				<input
					id="name"
					type="text"
					required
					bind:value={name}
					placeholder="Alex Crew Lead"
					class="h-14 w-full rounded-xl border-2 border-slate-800 bg-slate-900 px-4 text-base text-white placeholder-slate-600 transition-colors outline-none focus:border-amber-500"
				/>
			</div>

			<!-- Email Input -->
			<div>
				<label
					for="email"
					class="mb-2 block font-mono text-xs font-bold tracking-wider text-slate-300 uppercase"
				>
					Email Address
				</label>
				<input
					id="email"
					type="email"
					required
					bind:value={email}
					placeholder="crew@ultracrew.app"
					class="h-14 w-full rounded-xl border-2 border-slate-800 bg-slate-900 px-4 text-base text-white placeholder-slate-600 transition-colors outline-none focus:border-amber-500"
				/>
			</div>

			<!-- Password Input -->
			<div>
				<label
					for="password"
					class="mb-2 block font-mono text-xs font-bold tracking-wider text-slate-300 uppercase"
				>
					Password
				</label>
				<input
					id="password"
					type="password"
					required
					bind:value={password}
					placeholder="••••••••"
					class="h-14 w-full rounded-xl border-2 border-slate-800 bg-slate-900 px-4 text-base text-white placeholder-slate-600 transition-colors outline-none focus:border-amber-500"
				/>
			</div>

			<!-- Submit Button -->
			<button
				type="submit"
				disabled={loading}
				class="mt-2 flex h-16 w-full cursor-pointer items-center justify-center gap-2 rounded-xl border-b-4 border-amber-600 bg-amber-500 text-lg font-extrabold tracking-wider text-slate-950 uppercase shadow-lg shadow-amber-500/10 transition-all hover:bg-amber-400 active:scale-[0.98] active:bg-amber-600 disabled:opacity-50"
			>
				{#if loading}
					<span
						class="inline-block h-5 w-5 animate-spin rounded-full border-2 border-slate-950 border-t-transparent"
					></span>
					<span>Creating Account...</span>
				{:else}
					<span>Register</span>
				{/if}
			</button>
		</form>
	</main>

	<!-- Login Link Footer -->
	<footer class="mx-auto w-full max-w-md space-y-3 border-t border-slate-900 pt-4">
		<div class="text-center">
			<p class="mb-3 text-xs text-slate-500">Already have a crew account?</p>
			<a
				href={resolve('/login')}
				class="hover:bg-slate-850 flex h-14 w-full items-center justify-center rounded-xl border-2 border-slate-800 bg-slate-900 text-sm font-bold tracking-wider text-slate-200 uppercase transition-all hover:border-slate-700 active:scale-[0.98]"
			>
				Sign In Instead
			</a>
		</div>
	</footer>
</div>

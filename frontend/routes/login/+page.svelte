<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import type { UserResponse } from '$lib/types/UserResponse';
	import { user } from '$lib/user.svelte';
	import ChevronLeftIcon from '@lucide/svelte/icons/chevron-left';
	import MessageSquareWarningIcon from '@lucide/svelte/icons/message-square-warning';

	let email = $state('');
	let password = $state('');
	let errorMessage = $state('');
	let loading = $state(false);

	async function handleLogin(e: SubmitEvent) {
		e.preventDefault();
		errorMessage = '';
		loading = true;

		try {
			const response = await fetch('/api/auth/login', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({ email, password })
			});

			if (response.status === 200) {
				const body: UserResponse = await response.json();
				user.login(body.id);
				goto(resolve('/races'));
			} else {
				errorMessage = 'Login failed. Please check your credentials.';
			}
		} catch (err) {
			console.error(err);
			errorMessage = 'Login failed. Network or server connection error.';
		} finally {
			loading = false;
		}
	}
</script>

<div class="flex min-h-screen flex-col justify-between bg-base-200 p-6 font-sans text-base-content">
	<header class="mx-auto flex w-full max-w-md items-center justify-between pt-2">
		<a
			href={resolve('/')}
			class="btn gap-1 btn-ghost text-sm font-semibold btn-sm hover:bg-base-300"
		>
			<ChevronLeftIcon size="18"></ChevronLeftIcon>
			<span>Back</span>
		</a>
		<span class="font-mono text-xs font-semibold tracking-widest text-primary uppercase">
			Authentication
		</span>
	</header>

	<main class="mx-auto my-auto w-full max-w-md py-6">
		<div class="card bg-base-100 shadow-xl">
			<div class="card-body">
				<div class="mb-4">
					<h1 class="text-3xl font-black tracking-tight text-primary uppercase italic">
						Crew <span class="text-secondary">Sign In</span>
					</h1>
					<p class="mt-1 text-sm text-base-content/70">
						Enter your credentials to access your active race dashboards.
					</p>
				</div>

				{#if errorMessage}
					<div role="alert" class="alert py-2 text-sm alert-error">
						<MessageSquareWarningIcon size="20"></MessageSquareWarningIcon>
						<span>{errorMessage}</span>
					</div>
				{/if}

				<form onsubmit={handleLogin} class="space-y-4">
					<div class="form-control w-full">
						<label for="email" class="label pb-1">
							<span class="label-text font-medium">Email Address</span>
						</label>
						<input
							id="email"
							type="email"
							required
							bind:value={email}
							placeholder="runner@example.com"
							class="input-bordered input w-full"
						/>
					</div>

					<div class="form-control w-full">
						<label for="password" class="label pb-1">
							<span class="label-text font-medium">Password</span>
						</label>
						<input
							id="password"
							type="password"
							required
							bind:value={password}
							placeholder="••••••••"
							class="input-bordered input w-full"
						/>
					</div>

					<button type="submit" disabled={loading} class="btn mt-2 btn-block uppercase btn-primary">
						{#if loading}
							<span class="loading loading-sm loading-spinner"></span>
							<span>Authenticating...</span>
						{:else}
							<span>Sign In</span>
						{/if}
					</button>
				</form>

				<div class="divider my-4">OR</div>

				<div class="text-center">
					<p class="mb-3 text-xs text-base-content/70">Need a new crew account?</p>
					<a href={resolve('/register')} class="btn btn-block uppercase btn-neutral">
						Create New User
					</a>
				</div>
			</div>
		</div>
	</main>
</div>

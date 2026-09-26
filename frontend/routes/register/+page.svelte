<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import ChevronLeftIcon from '@lucide/svelte/icons/chevron-left';
	import MessageSquareWarningIcon from '@lucide/svelte/icons/message-square-warning';

	let name = $state('');
	let email = $state('');
	let password = $state('');
	let errorMessage = $state('');
	let loading = $state(false);

	async function handleRegister(e: SubmitEvent) {
		e.preventDefault();
		errorMessage = '';
		loading = true;

		try {
			const response = await fetch('/api/auth/register', {
				method: 'POST',
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
			New Account
		</span>
	</header>

	<main class="mx-auto my-auto w-full max-w-md py-6">
		<div class="card bg-base-100 shadow-xl">
			<div class="card-body">
				<div class="mb-4">
					<h1 class="text-3xl font-black tracking-tight text-primary uppercase italic">
						Create <span class="text-secondary">Account</span>
					</h1>
					<p class="mt-1 text-sm text-base-content/70">
						Sign up to manage and sync crew operations.
					</p>
				</div>

				{#if errorMessage}
					<div role="alert" class="alert py-2 text-sm alert-error">
						<MessageSquareWarningIcon size="20"></MessageSquareWarningIcon>
						<span>{errorMessage}</span>
					</div>
				{/if}

				<form onsubmit={handleRegister} class="space-y-4">
					<div class="form-control w-full">
						<label for="name" class="label pb-1">
							<span class="label-text font-medium">Full Name</span>
						</label>
						<input
							id="name"
							type="text"
							required
							bind:value={name}
							placeholder="Alex Crew Lead"
							class="input-bordered input w-full"
						/>
					</div>

					<div class="form-control w-full">
						<label for="email" class="label pb-1">
							<span class="label-text font-medium">Email Address</span>
						</label>
						<input
							id="email"
							type="email"
							required
							bind:value={email}
							placeholder="crew@ultracrew.app"
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
							<span>Creating Account...</span>
						{:else}
							<span>Register</span>
						{/if}
					</button>
				</form>

				<div class="divider my-4">OR</div>

				<div class="text-center">
					<p class="mb-3 text-xs text-base-content/70">Already have a crew account?</p>
					<a href={resolve('/login')} class="btn btn-block uppercase btn-neutral">
						Sign In Instead
					</a>
				</div>
			</div>
		</div>
	</main>
</div>

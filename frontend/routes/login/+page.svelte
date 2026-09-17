<script lang="ts">
  import { goto } from '$app/navigation';
  import { resolve } from '$app/paths';
  import { apiFetch } from '$lib/api';
	import type { UserResponse } from '$lib/types/UserResponse';
  import { user } from '$lib/user.svelte';

  let email = $state('');
  let password = $state('');
  let errorMessage = $state('');
  let loading = $state(false);

  async function handleLogin(e: SubmitEvent) {
    e.preventDefault();
    errorMessage = '';
    loading = true;

    try {
      const response = await apiFetch('/api/auth/login', {
        method: 'POST',
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
      class="btn btn-ghost btn-sm gap-1 text-sm font-semibold hover:bg-base-300"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="h-4 w-4"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
      >
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
      </svg>
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
          <div role="alert" class="alert alert-error py-2 text-sm">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-5 w-5 shrink-0 stroke-current"
              fill="none"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
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
              class="input input-bordered w-full"
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
              class="input input-bordered w-full"
            />
          </div>

          <button
            type="submit"
            disabled={loading}
            class="btn btn-primary btn-block uppercase mt-2"
          >
            {#if loading}
              <span class="loading loading-spinner loading-sm"></span>
              <span>Authenticating...</span>
            {:else}
              <span>Sign In</span>
            {/if}
          </button>
        </form>

        <div class="divider my-4">OR</div>

        <div class="text-center">
          <p class="mb-3 text-xs text-base-content/70">Need a new crew account?</p>
          <a
            href={resolve('/register')}
            class="btn btn-neutral btn-block uppercase"
          >
            Create New User
          </a>
        </div>
      </div>
    </div>
  </main>
</div>

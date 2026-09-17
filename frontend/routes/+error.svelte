<!-- src/routes/+error.svelte -->
<script lang="ts">
  import { page } from '$app/state';
  import { resolve } from '$app/paths';
</script>

<main class="hero bg-base-200 min-h-screen">
  <div class="hero-content text-center">
    <div class="max-w-md">
      {#if page.status === 404}
        <h1 class="text-5xl font-extrabold tracking-tight text-base-content sm:text-6xl">
          Page not found
        </h1>
        <p class="py-6 text-base text-base-content/70">
          Sorry, we couldn't find the page you're looking for. It might have been moved or deleted.
        </p>
      {:else}
        <h1 class="text-4xl font-extrabold tracking-tight text-base-content sm:text-5xl">
          Something went wrong
        </h1>
        <p class="py-6 text-base text-base-content/70">
          {page.error?.message ?? 'An unexpected error occurred. Please try again later.'}
        </p>
      {/if}

      <div class="flex flex-col sm:flex-row gap-3 justify-center">
        <a href={resolve('/')} class="btn btn-primary gap-2">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 00-1-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
          </svg>
          Back to Home
        </a>

        <button onclick={() => history.back()} class="btn btn-ghost">
          Go Back
        </button>
      </div>
    </div>
  </div>
</main>

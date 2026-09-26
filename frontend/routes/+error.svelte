<!-- src/routes/+error.svelte -->
<script lang="ts">
	import { page } from '$app/state';
	import { resolve } from '$app/paths';
	import HouseIcon from '@lucide/svelte/icons/house';
</script>

<main class="hero min-h-screen bg-base-200">
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

			<div class="flex flex-col justify-center gap-3 sm:flex-row">
				<a href={resolve('/')} class="btn gap-2 btn-primary">
					<HouseIcon></HouseIcon>
					Back to Home
				</a>

				<button onclick={() => history.back()} class="btn btn-ghost"> Go Back </button>
			</div>
		</div>
	</div>
</main>

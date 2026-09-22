<script lang="ts">
	import { goto } from "$app/navigation";
	import { resolve } from "$app/paths";
	import { apiFetch } from "$lib/api";
	import type { Race } from "$lib/types/Race";

	let name = $state('');
	let event_date = $state('');

	let isSubmitting = $state(false);
	let error = $state<string | null>(null);

	async function handleSubmit(event: Event) {
		event.preventDefault();
		isSubmitting = true;
		error = null;

		try {
			const payload = { name, event_date };

			const res = await apiFetch('/api/races', {
				method: 'POST',
				body: JSON.stringify(payload)
			});
			if (!res.ok) throw new Error('Failed to create race.');
      const result: Race = await res.json();
      await goto(resolve(`/races/${result.id}`));

			name = '';
			event_date = '';
		} catch (err: unknown) {
      if (err instanceof Error)
      {
        error = err.message || 'An unexpected error occurred.';
      }
		} finally {
			isSubmitting = false;
		}
	}
</script>

<div class="w-full max-w-lg mx-auto py-12 px-4">
	<div class="card bg-base-100 shadow-2xl border border-base-200">
		<div class="card-body">
			<h2 class="card-title text-2xl font-bold">Create New Race</h2>
			<p class="text-base-content/70 text-sm mb-4">
				Enter the details below to schedule a new racing event.
			</p>

			{#if error}
				<div class="alert alert-error mb-4 rounded-box">
					<svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
					<span>{error}</span>
				</div>
			{/if}

			<form onsubmit={handleSubmit} class="space-y-4">
				<fieldset class="form-control w-full">
					<label class="label" for="name">
						<span class="label-text font-medium">Race Name</span>
					</label>
					<input
						type="text"
						id="name"
						placeholder="e.g. Midnight City Sprint"
						class="input input-bordered w-full focus:input-primary"
						bind:value={name}
						required
					/>
				</fieldset>

				<fieldset class="form-control w-full">
					<label class="label" for="event_date">
						<span class="label-text font-medium">Event Date</span>
					</label>
					<input
						type="date"
						id="event_date"
						class="input input-bordered w-full focus:input-primary text-base-content"
						bind:value={event_date}
						required
					/>
				</fieldset>

				<div class="form-control mt-6">
					<button type="submit" class="btn btn-primary w-full" disabled={isSubmitting}>
						{#if isSubmitting}
							<span class="loading loading-spinner loading-sm"></span>
							Creating...
						{:else}
							Create Race
						{/if}
					</button>
				</div>
			</form>
		</div>
	</div>
</div>

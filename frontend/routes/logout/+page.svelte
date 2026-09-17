<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { resolve } from '$app/paths';
  import { user } from '$lib/user.svelte';
  import { apiFetch } from '$lib/api';

  onMount(async () => {
    try {
      await apiFetch('/api/logout', { method: 'POST' });
    } catch (err) {
      console.error('Logout error:', err);
    } finally {
      await user.logout();
      await goto(resolve('/'));
    }
  });
</script>

<div class="logout-screen">
  <p>Logging out, please wait...</p>
</div>

import type { User } from '$lib/types/User';
import { apiFetch } from '$lib/api';
import { user } from '$lib/user.svelte';
import { goto } from '$app/navigation';
import { resolve } from '$app/paths';

export const load = async () => {
	if (!user.id) {
		await goto(resolve('/login'));
		return;
	}

	const res = await apiFetch(`/api/users/${user.id}`);
	if (!res.ok) {
		await goto(resolve('/login'));
		return;
	}
	const loadedUser: User = await res.json();
	return {
		user: loadedUser
	};
};

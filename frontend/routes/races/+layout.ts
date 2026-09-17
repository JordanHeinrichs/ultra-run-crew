import type { User } from '$lib/types/User';
import { apiFetch } from '$lib/api';
import { user } from '$lib/user.svelte';
import { goto } from '$app/navigation';
import { resolve } from '$app/paths';

export const load = async () => {
	if (!user.id) {
		goto(resolve('/login'));
	}

	const res = await apiFetch(`/api/users/${user.id}`);
	if (!res.ok) {
		goto(resolve('/login'));
	}
	const loadedUser: User = await res.json();
	return {
		user: loadedUser
	};
};

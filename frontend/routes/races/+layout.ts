import type { User } from '$lib/types/User';
import { apiFetch } from '$lib/api';
import { user } from '$lib/user.svelte';
import { resolve } from '$app/paths';
import { redirect } from '@sveltejs/kit';

export const load = async () => {
	if (!user.id) {
		throw redirect(307, resolve('/login'));
	}

	const res = await apiFetch(`/api/users/${user.id}`);
	if (!res.ok) {
		throw redirect(307, resolve('/login'));
	}
	const loadedUser: User = await res.json();
	return {
		user: loadedUser
	};
};

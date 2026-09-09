import type { RaceListRace } from '$lib/types/RaceListRace';
import { apiFetch } from '$lib/api';
import { error } from '@sveltejs/kit';

export const load = async () => {
	const res = await apiFetch('/api/races');
	if (!res.ok) {
		error(res.status, {
			message: 'Failed to load races from server'
		});
	}

	const races: RaceListRace[] = await res.json();
	console.log(`Fetched races ${races}`);
	return {
		races
	};
};

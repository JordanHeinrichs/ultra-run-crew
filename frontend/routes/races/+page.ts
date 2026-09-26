import type { RaceListRace } from '$lib/types/RaceListRace';
import { error } from '@sveltejs/kit';

export const load = async ({ fetch }) => {
	const res = await fetch('/api/races');
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

import type { Race } from '$lib/types/Race';
import type { User } from '$lib/types/User.js';
import { error } from '@sveltejs/kit';

export const load = async ({ params, fetch }) => {
	const raceId = params.id;
	const res = await fetch(`/api/races/${raceId}`);
	if (!res.ok) {
		error(res.status, {
			message: 'Failed to load race from server'
		});
	}
	const race: Race = await res.json();
	const runnerRes = await fetch(`/api/users/${race.runner}`);
	if (!res.ok) {
		error(res.status, {
			message: 'Failed to load runner from server'
		});
	}
	const runner: User = await runnerRes.json();
	return {
		raceId,
		race,
		runner
	};
};

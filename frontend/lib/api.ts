import { resolve } from '$app/paths';
import { page } from '$app/state';
import { dev } from '$app/environment';
import { goto } from '$app/navigation';

export async function apiFetch(input: RequestInfo | URL, init?: RequestInit): Promise<Response> {
	const url = dev ? `http://127.0.0.1:3000${input}` : input;

	console.info(`Fetching ${url}`);
	const response = await fetch(url, {
		...init,
		credentials: 'include',
		headers: {
			'Content-Type': 'application/json',
			...init?.headers
		}
	});

	if (response.status === 401) {
		const currentPath = page.url.pathname;

		if (currentPath !== '/login') {
			const redirectTarget = encodeURIComponent(currentPath + page.url.search);
			await goto(resolve(`/login?redirectTo=${redirectTarget}`));
		}
	}

	return response;
}

import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vitest/config';
import { playwright } from '@vitest/browser-playwright';
import adapter from '@sveltejs/adapter-static';
import { sveltekit } from '@sveltejs/kit/vite';

export default defineConfig({
	plugins: [
		tailwindcss(),
		sveltekit({
			compilerOptions: {
				// Force runes mode for the project, except for libraries. Can be removed in svelte 6.
				runes: ({ filename }) =>
					filename.split(/[/\\]/).includes('node_modules') ? undefined : true
			},
			files: {
				appTemplate: 'frontend/app.html',
				errorTemplate: 'frontend/error.html',
				routes: 'frontend/routes',
				lib: 'frontend/lib',
				params: 'frontend/params',
				hooks: {
					client: 'frontend/hooks.client',
					server: 'frontend/hooks.server',
					universal: 'frontend/hooks'
				},
				serviceWorker: 'frontend/service-worker'
			},
			adapter: adapter({
				fallback: 'app.html'
			})
		})
	],
	test: {
		expect: { requireAssertions: true },
		projects: [
			{
				extends: './vite.config.ts',
				test: {
					name: 'client',
					browser: {
						enabled: true,
						provider: playwright(),
						instances: [{ browser: 'chromium', headless: true }]
					},
					include: ['frontend/**/*.svelte.{test,spec}.{js,ts}'],
					exclude: ['frontend/lib/server/**']
				}
			},

			{
				extends: './vite.config.ts',
				test: {
					name: 'server',
					environment: 'node',
					include: ['frontend/**/*.{test,spec}.{js,ts}'],
					exclude: ['frontend/**/*.svelte.{test,spec}.{js,ts}']
				}
			}
		]
	}
});

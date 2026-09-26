<script lang="ts">
	import { resolve } from '$app/paths';
	import UserIcon from '@lucide/svelte/icons/user';
	import SettingsIcon from '@lucide/svelte/icons/settings';
	import LogoutIcon from '@lucide/svelte/icons/log-out';
	import type { Snippet } from 'svelte';
	import type { LayoutData } from './$types';

	let { children, data }: { children: Snippet; data: LayoutData } = $props();

	function getInitials() {
		const firstLetters = data.user.name.split(' ').map((x) => x.slice(0, 1).toUpperCase());
		return firstLetters.length >= 2
			? `${firstLetters.at(0)}${firstLetters.at(-1)}`
			: data.user.name.slice(0, 2).toUpperCase();
	}
</script>

<header class="navbar bg-base-100 px-6 shadow-md backdrop-blur-md">
	<div class="flex-1">
		<a
			href={resolve('/races')}
			class="text-xl font-black tracking-tight text-primary uppercase italic"
		>
			Ultra<span class="text-secondary">Crew</span>
		</a>
	</div>

	<div class="flex-none">
		<div class="dropdown dropdown-end">
			<div
				tabindex="0"
				role="button"
				class="btn avatar avatar-placeholder btn-circle"
				aria-label="User menu"
			>
				<div class="w-10 rounded-full bg-primary text-primary-content">
					<span class="font-bold">{getInitials()}</span>
				</div>
			</div>
			<ul
				class="menu dropdown-content z-50 mt-3 w-52 menu-sm rounded-box border border-base-300 bg-base-100 p-2 shadow-lg"
			>
				<li class="menu-title px-3 py-1 text-xs font-bold text-base-content/50 uppercase">
					{data.user.name}
				</li>
				<li>
					<a href={resolve('/')} class="py-2">
						<UserIcon size="20"></UserIcon>
						Profile & Crew
					</a>
				</li>
				<li>
					<a href={resolve('/')} class="py-2">
						<SettingsIcon size="20"></SettingsIcon>
						Settings
					</a>
				</li>
				<div class="divider my-1"></div>
				<li>
					<a href={resolve('/logout')} class="py-2 text-error hover:bg-error/10">
						<LogoutIcon size="20"></LogoutIcon>
						Sign Out
					</a>
				</li>
			</ul>
		</div>
	</div>
</header>

<main class="mx-auto max-w-6xl p-4 sm:p-6">
	{@render children()}
</main>

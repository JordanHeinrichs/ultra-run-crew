<script lang="ts">
  import { resolve } from '$app/paths';
	import type { Snippet } from 'svelte';
	import type { LayoutData } from './$types';

  let { children, data }: { children: Snippet; data: LayoutData } = $props();

  function getInitials() {
    const firstLetters = data.user.name.split(' ').map(x => x.slice(0, 1).toUpperCase());
    return firstLetters.length >= 2 ? `${firstLetters.at(0)}${firstLetters.at(-1)}` : data.user.name.slice(0, 2).toUpperCase();
  }
</script>

<header class="navbar bg-base-100 shadow-md backdrop-blur-md px-6">
  <div class="flex-1">
    <a href={resolve('/races')} class="text-xl font-black italic tracking-tight text-primary uppercase">
      Ultra<span class="text-secondary">Crew</span>
    </a>
  </div>

  <div class="flex-none">
    <div class="dropdown dropdown-end">
      <div
        tabindex="0"
        role="button"
        class="btn btn-circle avatar avatar-placeholder"
        aria-label="User menu"
      >
        <div class="w-10 rounded-full bg-primary text-primary-content">
          <span class="font-bold">{getInitials()}</span>
        </div>
      </div>
      <ul
        class="menu menu-sm dropdown-content bg-base-100 rounded-box z-50 mt-3 w-52 border border-base-300 p-2 shadow-lg"
      >
        <li class="menu-title px-3 py-1 text-xs font-bold uppercase text-base-content/50">
          {data.user.name}
        </li>
        <li>
          <a href={resolve('/')} class="py-2">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
            </svg>
            Profile & Crew
          </a>
        </li>
        <li>
          <a href={resolve('/')} class="py-2">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
            Settings
          </a>
        </li>
        <div class="divider my-1"></div>
        <li>
          <a href={resolve('/logout')} class="text-error hover:bg-error/10 py-2">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
            </svg>
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

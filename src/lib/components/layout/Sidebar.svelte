<script lang="ts">
	import { onMount } from 'svelte';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { getVersion } from '@tauri-apps/api/app';
	import { cn } from '$lib/utils';
	import SyncButton from './SyncButton.svelte';
	import ThemeToggle from './ThemeToggle.svelte';
	import UpdateButton from './UpdateButton.svelte';

	const navItems = [
		{ href: '/', label: 'Dashboard', icon: '◆' },
		{ href: '/conversations', label: 'Conversations', icon: '◇' },
		{ href: '/search', label: 'Search', icon: '⌕' }
	];

	let version = $state('');

	onMount(async () => {
		version = await getVersion();
	});
</script>

<aside class="bg-card flex h-full w-56 shrink-0 flex-col border-r">
	<div class="flex items-center gap-2 border-b p-4">
		<img src="/app-icon.png" alt="App icon" class="h-6 w-6 rounded" />
		<h1 class="text-sm font-semibold tracking-tight">Claude Conversations</h1>
	</div>
	<nav class="flex-1 space-y-0.5 p-2">
		{#each navItems as item (item.href)}
			{@const isActive = item.href === '/' ? page.url.pathname === '/' : page.url.pathname.startsWith(item.href)}
			<a
				href={resolve(item.href)}
				class={cn(
					'flex items-center gap-2 rounded-md px-3 py-2 text-sm transition-colors',
					isActive
						? 'bg-primary text-primary-foreground'
						: 'text-muted-foreground hover:bg-accent hover:text-accent-foreground'
				)}
			>
				<span class="text-base">{item.icon}</span>
				{item.label}
			</a>
		{/each}
	</nav>
	<div class="text-muted-foreground flex items-center justify-between border-t p-3 text-xs">
		<span>{version ? ` v${version}` : ''}</span>
		<span class="inline-flex gap-1">
			<SyncButton />
			<UpdateButton />
			<ThemeToggle />
		</span>
	</div>
</aside>

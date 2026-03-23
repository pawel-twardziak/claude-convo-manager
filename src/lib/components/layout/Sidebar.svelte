<script lang="ts">
	import { page } from '$app/state';
	import { cn } from '$lib/utils';
	import SyncButton from './SyncButton.svelte';
	import ThemeToggle from './ThemeToggle.svelte';

	const navItems = [
		{ href: '/', label: 'Dashboard', icon: '◆' },
		{ href: '/conversations', label: 'Conversations', icon: '◇' },
		{ href: '/search', label: 'Search', icon: '⌕' }
	];
</script>

<aside class="w-56 shrink-0 border-r bg-card flex flex-col h-full">
	<div class="p-4 border-b flex items-center gap-2">
		<img src="/app-icon.png" alt="App icon" class="w-6 h-6 rounded" />
		<h1 class="font-semibold text-sm tracking-tight">Claude Conversations</h1>
	</div>
	<nav class="flex-1 p-2 space-y-0.5">
		{#each navItems as item}
			{@const isActive = item.href === '/' ? page.url.pathname === '/' : page.url.pathname.startsWith(item.href)}
			<a
				href={item.href}
				class={cn(
					'flex items-center gap-2 px-3 py-2 rounded-md text-sm transition-colors',
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
	<div class="p-3 border-t text-xs text-muted-foreground flex items-center justify-between">
		<span>Claude Code Manager</span>
		<span class="inline-flex gap-1">
			<SyncButton />
			<ThemeToggle />
		</span>
	</div>
</aside>

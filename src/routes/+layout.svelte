<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import Sidebar from '$lib/components/layout/Sidebar.svelte';
	import { loadDetectedApps } from '$lib/stores/ide.svelte';
	import { startSync, isSyncing, getSyncProgress } from '$lib/stores/sync.svelte';

	let { children } = $props();

	onMount(() => {
		loadDetectedApps();
		startSync();
	});
</script>

<Sidebar />
<main class="relative flex-1 overflow-auto">
	{@render children()}
	{#if isSyncing()}
		<div class="absolute inset-0 z-50 flex items-center justify-center bg-black/40 backdrop-blur-[2px]">
			<div class="bg-card flex flex-col items-center gap-3 rounded-lg border px-8 py-6 shadow-lg">
				<svg
					class="text-muted-foreground h-6 w-6 animate-spin"
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
				>
					<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
					<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
				</svg>
				<p class="text-sm font-medium">Syncing...</p>
				{#if getSyncProgress().phase}
					<p class="text-muted-foreground text-xs">{getSyncProgress().phase}</p>
				{/if}
				{#if getSyncProgress().total > 0}
					<p class="text-muted-foreground text-xs">
						{getSyncProgress().current} / {getSyncProgress().total}
					</p>
				{/if}
			</div>
		</div>
	{/if}
</main>

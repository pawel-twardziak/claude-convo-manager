<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { startSync, isSyncing } from '$lib/stores/sync.svelte';

	async function handleSync() {
		await startSync();
		await invalidateAll();
	}
</script>

<button
	onclick={handleSync}
	disabled={isSyncing()}
	title="Re-sync conversations from ~/.claude"
	class="h-7 px-2 text-[11px] rounded-md text-muted-foreground hover:bg-accent hover:text-accent-foreground disabled:opacity-50 cursor-pointer"
>
	{isSyncing() ? 'Syncing...' : 'Sync'}
</button>

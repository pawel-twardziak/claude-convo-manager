<script lang="ts">
	import { startSync, isSyncing } from '$lib/stores/sync.svelte';
</script>

<button
	onclick={startSync}
	disabled={isSyncing()}
	title="Re-sync conversations from ~/.claude"
	class="text-muted-foreground hover:bg-accent hover:text-accent-foreground h-7 cursor-pointer rounded-md px-2 text-[11px] disabled:opacity-50"
>
	{isSyncing() ? 'Syncing...' : 'Sync'}
</button>

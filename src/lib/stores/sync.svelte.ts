import { triggerSync, onSyncProgress } from '$lib/api/sync';

let syncing = $state(false);
let progress = $state({ phase: '', current: 0, total: 0 });
let syncVersion = $state(0);

export function isSyncing(): boolean {
	return syncing;
}

export function getSyncProgress() {
	return progress;
}

export function getSyncVersion(): number {
	return syncVersion;
}

export async function startSync(): Promise<{ sessions: number; messages: number }> {
	if (syncing) return { sessions: 0, messages: 0 };
	syncing = true;
	progress = { phase: 'Starting...', current: 0, total: 0 };

	const unlisten = await onSyncProgress((p) => {
		progress = p;
	});

	try {
		const result = await triggerSync();
		syncVersion++;
		return { sessions: result.sessions, messages: result.messages };
	} finally {
		syncing = false;
		unlisten();
	}
}

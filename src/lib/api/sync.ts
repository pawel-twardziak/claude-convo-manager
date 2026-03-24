import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

interface SyncResult {
	ok: boolean;
	sessions: number;
	messages: number;
}

interface SyncProgress {
	phase: string;
	current: number;
	total: number;
}

export async function triggerSync(): Promise<SyncResult> {
	return invoke<SyncResult>('trigger_sync');
}

export async function onSyncProgress(callback: (progress: SyncProgress) => void): Promise<UnlistenFn> {
	return listen<SyncProgress>('sync-progress', (event) => {
		callback(event.payload);
	});
}

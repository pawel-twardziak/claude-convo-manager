import { check, type Update } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';

type UpdateStatus = 'idle' | 'checking' | 'available' | 'downloading' | 'ready' | 'error' | 'up-to-date';

let status = $state<UpdateStatus>('idle');
let update = $state<Update | null>(null);
let errorMessage = $state('');
let downloadProgress = $state(0);

export function getUpdateStatus(): UpdateStatus {
	return status;
}

export function getUpdateVersion(): string | undefined {
	return update?.version;
}

export function getDownloadProgress(): number {
	return downloadProgress;
}

export function getErrorMessage(): string {
	return errorMessage;
}

export async function checkForUpdate(): Promise<void> {
	if (status === 'checking' || status === 'downloading') return;

	status = 'checking';
	errorMessage = '';

	try {
		const result = await check();
		if (result) {
			update = result;
			status = 'available';
		} else {
			status = 'up-to-date';
		}
	} catch (e) {
		status = 'error';
		errorMessage = e instanceof Error ? e.message : String(e);
	}
}

export async function downloadAndInstall(): Promise<void> {
	if (!update || status === 'downloading') return;

	status = 'downloading';
	downloadProgress = 0;

	try {
		let totalBytes = 0;
		let downloadedBytes = 0;

		await update.downloadAndInstall((progress) => {
			if (progress.event === 'Started' && progress.data.contentLength) {
				totalBytes = progress.data.contentLength;
			} else if (progress.event === 'Progress') {
				downloadedBytes += progress.data.chunkLength;
				downloadProgress = totalBytes > 0 ? Math.round((downloadedBytes / totalBytes) * 100) : 0;
			} else if (progress.event === 'Finished') {
				downloadProgress = 100;
			}
		});

		status = 'ready';
	} catch (e) {
		status = 'error';
		errorMessage = e instanceof Error ? e.message : String(e);
	}
}

export async function restartApp(): Promise<void> {
	await relaunch();
}

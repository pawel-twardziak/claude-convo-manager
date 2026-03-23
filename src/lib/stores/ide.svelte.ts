import { detectAvailableApps, type DetectedApp } from '$lib/api/ide';

let apps = $state<DetectedApp[]>([]);
let loaded = $state(false);

export async function loadDetectedApps(): Promise<void> {
	if (loaded) return;
	try {
		apps = await detectAvailableApps();
	} catch (e) {
		console.error('Failed to detect apps:', e);
		apps = [];
	}
	loaded = true;
}

export function getDetectedIDEs(): DetectedApp[] {
	return apps.filter((a) => a.app_type === 'ide');
}

export function getDetectedTerminals(): DetectedApp[] {
	return apps.filter((a) => a.app_type === 'terminal');
}

export function hasDetectedApps(): boolean {
	return apps.length > 0;
}

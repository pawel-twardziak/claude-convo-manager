import { invoke } from '@tauri-apps/api/core';

export interface DetectedApp {
	id: string;
	name: string;
	app_type: 'ide' | 'terminal';
	command: string;
}

export async function detectAvailableApps(): Promise<DetectedApp[]> {
	return invoke<DetectedApp[]>('detect_available_apps');
}

export async function openInApp(appId: string, path: string): Promise<void> {
	return invoke<void>('open_in_app', { appId, path });
}

export async function openTerminal(terminalId: string, path: string, command?: string): Promise<void> {
	return invoke<void>('open_terminal', { terminalId, path, command: command ?? null });
}

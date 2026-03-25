import { invoke } from '@tauri-apps/api/core';
import type { SessionWithProject, FilterOptions } from '$lib/types/db';

interface GetSessionsParams {
	projectId?: number;
	gitBranch?: string;
	model?: string;
	dateFrom?: string;
	dateTo?: string;
	search?: string;
	sortBy?: string;
	sortDir?: 'asc' | 'desc';
	page?: number;
	pageSize?: number;
}

interface GetSessionsResponse {
	sessions: SessionWithProject[];
	total: number;
	page: number;
	pageSize: number;
}

export async function getSessions(params: GetSessionsParams = {}): Promise<GetSessionsResponse> {
	return invoke<GetSessionsResponse>('get_sessions', { params });
}

export async function getSession(sessionId: string): Promise<SessionWithProject | null> {
	return invoke<SessionWithProject | null>('get_session', { sessionId });
}

export async function getFilterOptions(): Promise<FilterOptions> {
	return invoke<FilterOptions>('get_filter_options');
}

export async function renameSession(sessionId: string, newTitle: string): Promise<void> {
	return invoke<void>('rename_session', { sessionId, newTitle });
}

export async function cloneSession(sessionId: string, targetProjectId: number): Promise<string> {
	return invoke<string>('clone_session', { sessionId, targetProjectId });
}

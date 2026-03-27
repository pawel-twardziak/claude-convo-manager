import { invoke } from '@tauri-apps/api/core';
import type { MessageRow } from '$lib/types/db';

interface GetMessagesParams {
	sessionId: string;
	offset?: number;
	limit?: number;
	excludeSidechain?: boolean;
}

interface GetMessagesResponse {
	messages: MessageRow[];
	total: number;
	hasMore: boolean;
}

export async function getSessionMessages(params: GetMessagesParams): Promise<GetMessagesResponse> {
	return invoke<GetMessagesResponse>('get_session_messages', { params });
}

interface ReplaceAllParams {
	sessionId: string;
	searchTerm: string;
	replaceTerm: string;
	caseSensitive?: boolean;
}

interface ReplaceOneParams extends ReplaceAllParams {
	lineNumber: number;
	occurrenceIndex: number;
}

interface ReplaceResult {
	replacedCount: number;
}

export async function replaceInSession(params: ReplaceAllParams): Promise<ReplaceResult> {
	return invoke<ReplaceResult>('replace_in_session', { params });
}

export async function replaceOneInSession(params: ReplaceOneParams): Promise<ReplaceResult> {
	return invoke<ReplaceResult>('replace_one_in_session', { params });
}

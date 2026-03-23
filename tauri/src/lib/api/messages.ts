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

import { invoke } from '@tauri-apps/api/core';
import type { SearchResult } from '$lib/types/db';

interface SearchParams {
	query: string;
	projectId?: number;
	limit?: number;
	offset?: number;
}

interface SearchResponse {
	results: SearchResult[];
	total: number;
}

export async function searchMessages(params: SearchParams): Promise<SearchResponse> {
	return invoke<SearchResponse>('search_messages', { params });
}

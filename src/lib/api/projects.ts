import { invoke } from '@tauri-apps/api/core';
import type { ProjectWithStats } from '$lib/types/db';

interface GetProjectsParams {
	search?: string;
	sortBy?: string;
	sortDir?: 'asc' | 'desc';
	page?: number;
	pageSize?: number;
}

interface GetProjectsResponse {
	projects: ProjectWithStats[];
	total: number;
	page: number;
	pageSize: number;
}

export async function getProjects(params: GetProjectsParams = {}): Promise<GetProjectsResponse> {
	return invoke<GetProjectsResponse>('get_projects', { params });
}

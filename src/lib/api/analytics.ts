import { invoke } from '@tauri-apps/api/core';
import type { DashboardStats } from '$lib/types/db';

interface TokenUsageEntry {
	date: string;
	inputTokens: number;
	outputTokens: number;
	cost: number;
	sessionCount: number;
}

interface ProjectBreakdownEntry {
	name: string | null;
	sessions: number;
	tokens: number;
}

interface ActivityEntry {
	date: string;
	count: number;
}

export async function getDashboardStats(): Promise<DashboardStats> {
	return invoke<DashboardStats>('get_dashboard_stats');
}

export async function getTokenUsageOverTime(
	params: {
		groupBy?: 'day' | 'week' | 'month';
		dateFrom?: string;
		dateTo?: string;
		projectId?: number;
	} = {}
): Promise<TokenUsageEntry[]> {
	return invoke<TokenUsageEntry[]>('get_token_usage_over_time', { params });
}

export async function getProjectBreakdown(): Promise<ProjectBreakdownEntry[]> {
	return invoke<ProjectBreakdownEntry[]>('get_project_breakdown');
}

export async function getActivityData(): Promise<ActivityEntry[]> {
	return invoke<ActivityEntry[]>('get_activity_data');
}

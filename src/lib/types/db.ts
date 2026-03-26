export interface ProjectRow {
	id: number;
	encoded_name: string;
	project_path: string;
	display_name: string | null;
	session_count: number;
	total_tokens: number;
	last_activity_at: string | null;
	created_at: string;
}

export interface ProjectWithStats {
	id: number;
	encodedName: string;
	projectPath: string;
	displayName: string | null;
	sessionCount: number;
	totalTokens: number;
	lastActivityAt: string | null;
	createdAt: string | null;
	totalInputTokens: number;
	totalOutputTokens: number;
	totalCacheCreationTokens: number;
	totalCacheReadTokens: number;
	estimatedCostUsd: number;
	distinctModels: string | null;
	distinctBranches: string | null;
}

export interface SessionRow {
	id: string; // UUID
	project_id: number;
	file_path: string;
	file_mtime: number | null;
	file_size: number | null;
	first_prompt: string | null;
	custom_title: string | null;
	message_count: number;
	user_message_count: number;
	assistant_message_count: number;
	tool_use_count: number;
	git_branch: string | null;
	cwd: string | null;
	model: string | null;
	version: string | null;
	permission_mode: string | null;
	is_sidechain: number;
	is_active: number;
	total_input_tokens: number;
	total_output_tokens: number;
	total_cache_creation_tokens: number;
	total_cache_read_tokens: number;
	estimated_cost_usd: number;
	created_at: string | null;
	modified_at: string | null;
	synced_at: string;
}

export interface SessionWithProject extends SessionRow {
	project_path: string;
	project_display_name: string | null;
}

export interface MessageRow {
	id: number;
	uuid: string | null;
	session_id: string;
	parent_uuid: string | null;
	type: string;
	role: string | null;
	is_sidechain: number;
	agent_id: string | null;
	model: string | null;
	content_text: string | null;
	content_json: string | null;
	has_tool_use: number;
	has_thinking: number;
	tool_names: string | null;
	input_tokens: number;
	output_tokens: number;
	stop_reason: string | null;
	timestamp: string | null;
	line_number: number | null;
}

export interface TagRow {
	id: number;
	name: string;
	color: string;
}

export interface SessionMetadataRow {
	session_id: string;
	is_favorite: number;
	notes: string | null;
	updated_at: string;
}

export interface SearchResult {
	message_id: number;
	session_id: string;
	project_display_name: string | null;
	session_title: string | null;
	snippet: string;
	timestamp: string | null;
	rank: number;
}

export interface DashboardStats {
	totalSessions: number;
	totalProjects: number;
	totalMessages: number;
	totalInputTokens: number;
	totalOutputTokens: number;
	totalCacheCreationTokens: number;
	totalCacheReadTokens: number;
	estimatedTotalCost: number;
	activeSessions: number;
	todaySessions: number;
	avgMessageCount: number;
}

export interface FilterOptions {
	projects: { id: number; displayName: string; sessionCount: number }[];
	branches: string[];
	models: string[];
	dateRange: { min: string; max: string };
}

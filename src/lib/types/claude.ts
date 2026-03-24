// Raw JSONL line types from ~/.claude/ conversation files

export interface ClaudeHistoryEntry {
	display: string;
	timestamp: number; // Unix ms
	project: string; // Absolute path
	pastedContents: Record<string, unknown>;
	sessionId?: string; // UUID, present in newer entries
}

export interface SessionsIndexFile {
	version: number;
	entries: SessionsIndexEntry[];
}

export interface SessionsIndexEntry {
	sessionId: string;
	fullPath: string;
	fileMtime: number;
	firstPrompt: string;
	messageCount: number;
	created: string; // ISO-8601
	modified: string; // ISO-8601
	gitBranch: string;
	projectPath: string;
	isSidechain: boolean;
}

export interface ActiveSessionFile {
	pid: number;
	sessionId: string;
	cwd: string;
	startedAt: number; // Unix ms
}

export interface SubagentMeta {
	agentType: string;
	description?: string;
}

// Message types in conversation JSONL files

export interface ThinkingBlock {
	type: 'thinking';
	thinking: string;
	signature: string;
}

export interface TextBlock {
	type: 'text';
	text: string;
}

export interface ToolUseBlock {
	type: 'tool_use';
	id: string;
	name: string;
	input: Record<string, unknown>;
	caller?: { type: string };
}

export interface ToolResultBlock {
	type: 'tool_result';
	tool_use_id: string;
	content: string;
	is_error?: boolean;
}

export type ContentBlock = ThinkingBlock | TextBlock | ToolUseBlock | ToolResultBlock;

export interface TokenUsage {
	input_tokens: number;
	output_tokens: number;
	cache_creation_input_tokens?: number;
	cache_read_input_tokens?: number;
	cache_creation?: {
		ephemeral_5m_input_tokens: number;
		ephemeral_1h_input_tokens: number;
	};
	service_tier?: string;
	inference_geo?: string;
	server_tool_use?: {
		web_search_requests: number;
		web_fetch_requests: number;
	};
	iterations?: unknown[];
	speed?: string;
}

export interface ClaudeUserMessage {
	type: 'user';
	parentUuid: string | null;
	isSidechain: boolean;
	userType: string;
	cwd: string;
	sessionId: string;
	version: string;
	gitBranch: string;
	message: {
		role: 'user';
		content: string | ToolResultBlock[];
	};
	uuid: string;
	timestamp: string; // ISO-8601
	permissionMode?: string;
	promptId?: string;
	sourceToolAssistantUUID?: string;
	toolUseResult?: {
		stdout: string;
		stderr: string;
		interrupted: boolean;
		isImage: boolean;
		noOutputExpected: boolean;
	};
	agentId?: string;
	isMeta?: boolean;
}

export interface ClaudeAssistantMessage {
	type: 'assistant';
	parentUuid: string;
	isSidechain: boolean;
	message: {
		model: string;
		id: string;
		type: 'message';
		role: 'assistant';
		content: ContentBlock[];
		stop_reason: string | null;
		stop_sequence: string | null;
		usage: TokenUsage;
	};
	requestId: string;
	uuid: string;
	timestamp: string; // ISO-8601
	userType: string;
	cwd: string;
	sessionId: string;
	version: string;
	gitBranch: string;
	agentId?: string;
	permissionMode?: string;
}

export interface ClaudeProgressMessage {
	type: 'progress';
	parentUuid: string;
	isSidechain: boolean;
	data: {
		type?: 'agent_progress';
		message: unknown;
		prompt?: string;
		agentId?: string;
	};
	toolUseID?: string;
	parentToolUseID?: string;
	uuid: string;
	timestamp: string;
	sessionId: string;
	slug?: string;
}

export interface ClaudeSystemMessage {
	type: 'system';
	subtype?: string;
	durationMs?: number;
	content?: string;
	uuid?: string;
	timestamp?: string;
	sessionId?: string;
}

export interface ClaudeLastPromptMessage {
	type: 'last-prompt';
	lastPrompt: string;
	sessionId: string;
}

export interface ClaudeFileHistorySnapshot {
	type: 'file-history-snapshot';
	messageId: string;
	snapshot: {
		messageId: string;
		trackedFileBackups: Record<string, unknown>;
		timestamp: string;
	};
	isSnapshotUpdate: boolean;
}

export interface ClaudeCustomTitleMessage {
	type: 'custom-title';
	customTitle: string;
	sessionId: string;
}

export interface ClaudeAgentNameMessage {
	type: 'agent-name';
	agentName: string;
	sessionId: string;
}

export type ClaudeMessage =
	| ClaudeUserMessage
	| ClaudeAssistantMessage
	| ClaudeProgressMessage
	| ClaudeSystemMessage
	| ClaudeLastPromptMessage
	| ClaudeFileHistorySnapshot
	| ClaudeCustomTitleMessage
	| ClaudeAgentNameMessage;

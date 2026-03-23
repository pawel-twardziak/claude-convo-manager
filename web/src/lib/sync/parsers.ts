import fs from "fs";
import readline from "readline";
import type {
  ClaudeMessage,
  ClaudeAssistantMessage,
  ClaudeUserMessage,
  ContentBlock,
} from "@/types/claude";

export interface ParsedMessage {
  uuid: string | null;
  parentUuid: string | null;
  type: string;
  role: string | null;
  isSidechain: boolean;
  agentId: string | null;
  model: string | null;
  contentText: string;
  contentJson: string | null;
  hasToolUse: boolean;
  hasThinking: boolean;
  toolNames: string[];
  inputTokens: number;
  outputTokens: number;
  cacheCreationTokens: number;
  cacheReadTokens: number;
  stopReason: string | null;
  timestamp: string | null;
  lineNumber: number;
}

export interface SessionParseResult {
  messages: ParsedMessage[];
  metadata: {
    firstPrompt: string | null;
    customTitle: string | null;
    models: Map<string, number>;
    totalInputTokens: number;
    totalOutputTokens: number;
    totalCacheCreationTokens: number;
    totalCacheReadTokens: number;
    toolUseCount: number;
    userMessageCount: number;
    assistantMessageCount: number;
    gitBranch: string | null;
    cwd: string | null;
    version: string | null;
    permissionMode: string | null;
    createdAt: string | null;
    modifiedAt: string | null;
  };
}

function extractContentText(content: string | ContentBlock[]): string {
  if (typeof content === "string") return content;
  if (!Array.isArray(content)) return "";

  const parts: string[] = [];
  for (const block of content) {
    if (block.type === "text") {
      parts.push(block.text);
    } else if (block.type === "tool_result") {
      if (
        typeof block.content === "string" &&
        block.content.length < 1000 &&
        !block.is_error
      ) {
        parts.push(block.content);
      } else if (block.is_error && typeof block.content === "string") {
        parts.push(`[Error: ${block.content.slice(0, 200)}]`);
      }
    } else if (block.type === "tool_use") {
      parts.push(`[Tool: ${block.name}]`);
    }
    // Skip thinking blocks from content_text
  }
  return parts.join("\n");
}

function extractAssistantInfo(msg: ClaudeAssistantMessage) {
  const content = msg.message?.content;
  if (!Array.isArray(content))
    return {
      contentText: "",
      hasToolUse: false,
      hasThinking: false,
      toolNames: [] as string[],
    };

  const textParts: string[] = [];
  const toolNames: string[] = [];
  let hasToolUse = false;
  let hasThinking = false;

  for (const block of content) {
    if (block.type === "text") {
      textParts.push(block.text);
    } else if (block.type === "tool_use") {
      hasToolUse = true;
      toolNames.push(block.name);
    } else if (block.type === "thinking") {
      hasThinking = true;
    }
  }

  return { contentText: textParts.join("\n"), hasToolUse, hasThinking, toolNames };
}

function isStreamingPartial(msg: ClaudeAssistantMessage): boolean {
  return (
    msg.message?.stop_reason === null &&
    msg.message?.usage?.inference_geo === "not_available"
  );
}

export async function parseSessionFile(
  filePath: string
): Promise<SessionParseResult> {
  const messages: ParsedMessage[] = [];
  const metadata: SessionParseResult["metadata"] = {
    firstPrompt: null,
    customTitle: null,
    models: new Map(),
    totalInputTokens: 0,
    totalOutputTokens: 0,
    totalCacheCreationTokens: 0,
    totalCacheReadTokens: 0,
    toolUseCount: 0,
    userMessageCount: 0,
    assistantMessageCount: 0,
    gitBranch: null,
    cwd: null,
    version: null,
    permissionMode: null,
    createdAt: null,
    modifiedAt: null,
  };

  // Track assistant message.id for deduplication
  const assistantMessageIds = new Map<string, number>(); // message.id -> index in messages[]
  let lineNumber = 0;

  const fileStream = fs.createReadStream(filePath, { encoding: "utf-8" });
  const rl = readline.createInterface({ input: fileStream, crlfDelay: Infinity });

  for await (const line of rl) {
    lineNumber++;
    if (!line.trim()) continue;

    let parsed: ClaudeMessage;
    try {
      parsed = JSON.parse(line);
    } catch {
      continue; // Skip malformed lines
    }

    // Skip types we don't store
    if (
      parsed.type === "progress" ||
      parsed.type === "file-history-snapshot" ||
      parsed.type === "last-prompt"
    ) {
      continue;
    }

    // Handle custom-title
    if (parsed.type === "custom-title") {
      metadata.customTitle = parsed.customTitle;
      continue;
    }

    // Handle agent-name as fallback title
    if (parsed.type === "agent-name") {
      if (!metadata.customTitle) {
        metadata.customTitle = parsed.agentName;
      }
      continue;
    }

    // Handle system messages (extract duration)
    if (parsed.type === "system") {
      continue;
    }

    // Handle user messages
    if (parsed.type === "user") {
      const userMsg = parsed as ClaudeUserMessage;

      // Skip meta/command messages
      if (userMsg.isMeta) continue;

      const contentText = extractContentText(userMsg.message?.content);

      // Track first user message that isn't a tool result
      if (
        !metadata.firstPrompt &&
        typeof userMsg.message?.content === "string" &&
        !userMsg.isSidechain
      ) {
        metadata.firstPrompt = userMsg.message.content.slice(0, 500);
      }

      // Track metadata from first non-sidechain user message
      if (!userMsg.isSidechain) {
        if (!metadata.gitBranch && userMsg.gitBranch) {
          metadata.gitBranch = userMsg.gitBranch;
        }
        if (!metadata.cwd && userMsg.cwd) {
          metadata.cwd = userMsg.cwd;
        }
        if (!metadata.version && userMsg.version) {
          metadata.version = userMsg.version;
        }
        if (!metadata.permissionMode && userMsg.permissionMode) {
          metadata.permissionMode = userMsg.permissionMode;
        }
      }

      // Track timestamps
      if (userMsg.timestamp) {
        if (!metadata.createdAt || userMsg.timestamp < metadata.createdAt) {
          metadata.createdAt = userMsg.timestamp;
        }
        if (!metadata.modifiedAt || userMsg.timestamp > metadata.modifiedAt) {
          metadata.modifiedAt = userMsg.timestamp;
        }
      }

      // Determine role based on content type
      const isToolResult = Array.isArray(userMsg.message?.content);

      if (!userMsg.isSidechain && !isToolResult) {
        metadata.userMessageCount++;
      }

      messages.push({
        uuid: userMsg.uuid || null,
        parentUuid: userMsg.parentUuid || null,
        type: "user",
        role: "user",
        isSidechain: userMsg.isSidechain,
        agentId: userMsg.agentId || null,
        model: null,
        contentText,
        contentJson: isToolResult
          ? JSON.stringify(userMsg.message.content)
          : null,
        hasToolUse: false,
        hasThinking: false,
        toolNames: [],
        inputTokens: 0,
        outputTokens: 0,
        cacheCreationTokens: 0,
        cacheReadTokens: 0,
        stopReason: null,
        timestamp: userMsg.timestamp || null,
        lineNumber,
      });
      continue;
    }

    // Handle assistant messages
    if (parsed.type === "assistant") {
      const assistantMsg = parsed as ClaudeAssistantMessage;
      const msgId = assistantMsg.message?.id;

      // Deduplication: skip streaming partials
      if (isStreamingPartial(assistantMsg) && msgId) {
        // Store index in case final message comes later
        // We'll add it now but may replace it
        const existingIdx = assistantMessageIds.get(msgId);
        if (existingIdx !== undefined) continue; // Already have a better version
      }

      const { contentText, hasToolUse, hasThinking, toolNames } =
        extractAssistantInfo(assistantMsg);

      const usage = assistantMsg.message?.usage;
      const inputTokens = usage?.input_tokens || 0;
      const outputTokens = usage?.output_tokens || 0;
      const cacheCreation = usage?.cache_creation_input_tokens || 0;
      const cacheRead = usage?.cache_read_input_tokens || 0;

      const model = assistantMsg.message?.model || null;
      const stopReason = assistantMsg.message?.stop_reason || null;

      const parsedMsg: ParsedMessage = {
        uuid: assistantMsg.uuid || null,
        parentUuid: assistantMsg.parentUuid || null,
        type: "assistant",
        role: "assistant",
        isSidechain: assistantMsg.isSidechain,
        agentId: assistantMsg.agentId || null,
        model,
        contentText,
        contentJson: JSON.stringify(assistantMsg.message?.content || []),
        hasToolUse,
        hasThinking,
        toolNames,
        inputTokens,
        outputTokens,
        cacheCreationTokens: cacheCreation,
        cacheReadTokens: cacheRead,
        stopReason,
        timestamp: assistantMsg.timestamp || null,
        lineNumber,
      };

      // Dedup: if we already have a message with same id, replace if this one is final
      if (msgId) {
        const existingIdx = assistantMessageIds.get(msgId);
        if (existingIdx !== undefined) {
          // Replace previous entry if this one has stop_reason (is final)
          if (stopReason !== null) {
            messages[existingIdx] = parsedMsg;
          }
          continue;
        }
        assistantMessageIds.set(msgId, messages.length);
      }

      // Track metadata
      if (!assistantMsg.isSidechain) {
        metadata.assistantMessageCount++;
        if (hasToolUse) metadata.toolUseCount += toolNames.length;
        metadata.totalInputTokens += inputTokens;
        metadata.totalOutputTokens += outputTokens;
        metadata.totalCacheCreationTokens += cacheCreation;
        metadata.totalCacheReadTokens += cacheRead;

        if (model) {
          metadata.models.set(model, (metadata.models.get(model) || 0) + 1);
        }
      }

      if (assistantMsg.timestamp) {
        if (
          !metadata.modifiedAt ||
          assistantMsg.timestamp > metadata.modifiedAt
        ) {
          metadata.modifiedAt = assistantMsg.timestamp;
        }
      }

      messages.push(parsedMsg);
    }
  }

  return { messages, metadata };
}

export function getPrimaryModel(models: Map<string, number>): string | null {
  if (models.size === 0) return null;
  let maxCount = 0;
  let primary: string | null = null;
  for (const [model, count] of models) {
    if (count > maxCount) {
      maxCount = count;
      primary = model;
    }
  }
  return primary;
}

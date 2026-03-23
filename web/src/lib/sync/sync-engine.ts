import fs from "fs";
import path from "path";
import readline from "readline";
import Database from "better-sqlite3";
import { getDb } from "@/lib/db/connection";
import {
  getClaudeDir,
  getProjectsDir,
  extractDisplayName,
} from "./path-encoder";
import { parseSessionFile, getPrimaryModel } from "./parsers";
import { estimateCost } from "./token-calculator";
import type {
  ClaudeHistoryEntry,
  SessionsIndexFile,
  ActiveSessionFile,
  SubagentMeta,
} from "@/types/claude";

const BATCH_SIZE = 500;

interface SyncProgress {
  phase: string;
  current: number;
  total: number;
}

type ProgressCallback = (progress: SyncProgress) => void;

export async function fullSync(onProgress?: ProgressCallback): Promise<void> {
  const db = getDb();
  const claudeDir = getClaudeDir();
  const projectsDir = getProjectsDir();

  // Phase 1: Parse history.jsonl to build project path map
  onProgress?.({ phase: "Reading history", current: 0, total: 1 });
  const projectPathMap = new Map<string, string>(); // encodedName -> projectPath
  const historyPath = path.join(claudeDir, "history.jsonl");

  if (fs.existsSync(historyPath)) {
    const fileStream = fs.createReadStream(historyPath, { encoding: "utf-8" });
    const rl = readline.createInterface({
      input: fileStream,
      crlfDelay: Infinity,
    });
    for await (const line of rl) {
      if (!line.trim()) continue;
      try {
        const entry: ClaudeHistoryEntry = JSON.parse(line);
        if (entry.project) {
          const encoded = entry.project.replace(/[/.]/g, "-");
          projectPathMap.set(encoded, entry.project);
        }
      } catch {
        continue;
      }
    }
  }

  // Phase 2: Discover project directories
  onProgress?.({ phase: "Discovering projects", current: 0, total: 1 });

  if (!fs.existsSync(projectsDir)) {
    console.log("No projects directory found at", projectsDir);
    return;
  }

  const projectDirs = fs
    .readdirSync(projectsDir, { withFileTypes: true })
    .filter((d) => d.isDirectory())
    .map((d) => d.name);

  // Phase 3: Create/update project records
  const insertProject = db.prepare(`
    INSERT INTO projects (encoded_name, project_path, display_name)
    VALUES (?, ?, ?)
    ON CONFLICT(encoded_name) DO UPDATE SET
      project_path = excluded.project_path,
      display_name = excluded.display_name
  `);

  const getProjectId = db.prepare(
    `SELECT id FROM projects WHERE encoded_name = ?`
  );

  db.exec("BEGIN TRANSACTION");
  for (const encodedName of projectDirs) {
    const projectPath = projectPathMap.get(encodedName) || `/${encodedName.replace(/^-/, "").replace(/-/g, "/")}`;
    const displayName = extractDisplayName(projectPath);
    insertProject.run(encodedName, projectPath, displayName);
  }
  db.exec("COMMIT");

  // Also enrich project paths from sessions-index.json where available
  for (const encodedName of projectDirs) {
    const indexPath = path.join(projectsDir, encodedName, "sessions-index.json");
    if (fs.existsSync(indexPath)) {
      try {
        const indexData: SessionsIndexFile = JSON.parse(
          fs.readFileSync(indexPath, "utf-8")
        );
        if (indexData.entries?.[0]?.projectPath) {
          const realPath = indexData.entries[0].projectPath;
          const displayName = extractDisplayName(realPath);
          insertProject.run(encodedName, realPath, displayName);
        }
      } catch {
        // Skip malformed index files
      }
    }
  }

  // Phase 4: Discover and parse session files
  const sessionFiles: { projectEncoded: string; sessionId: string; filePath: string }[] = [];

  for (const encodedName of projectDirs) {
    const projectDir = path.join(projectsDir, encodedName);
    const files = fs.readdirSync(projectDir).filter((f) => f.endsWith(".jsonl"));
    for (const file of files) {
      const sessionId = file.replace(".jsonl", "");
      sessionFiles.push({
        projectEncoded: encodedName,
        sessionId,
        filePath: path.join(projectDir, file),
      });
    }
  }

  onProgress?.({
    phase: "Parsing sessions",
    current: 0,
    total: sessionFiles.length,
  });

  // Prepare statements for batch inserts
  const insertSession = db.prepare(`
    INSERT INTO sessions (
      id, project_id, file_path, file_mtime, file_size,
      first_prompt, custom_title, message_count,
      user_message_count, assistant_message_count, tool_use_count,
      git_branch, cwd, model, version, permission_mode,
      is_sidechain, total_input_tokens, total_output_tokens,
      total_cache_creation_tokens, total_cache_read_tokens,
      estimated_cost_usd, created_at, modified_at
    ) VALUES (
      ?, ?, ?, ?, ?,
      ?, ?, ?,
      ?, ?, ?,
      ?, ?, ?, ?, ?,
      ?, ?, ?,
      ?, ?,
      ?, ?, ?
    ) ON CONFLICT(id) DO UPDATE SET
      file_mtime = excluded.file_mtime,
      file_size = excluded.file_size,
      first_prompt = excluded.first_prompt,
      custom_title = excluded.custom_title,
      message_count = excluded.message_count,
      user_message_count = excluded.user_message_count,
      assistant_message_count = excluded.assistant_message_count,
      tool_use_count = excluded.tool_use_count,
      git_branch = excluded.git_branch,
      cwd = excluded.cwd,
      model = excluded.model,
      version = excluded.version,
      permission_mode = excluded.permission_mode,
      total_input_tokens = excluded.total_input_tokens,
      total_output_tokens = excluded.total_output_tokens,
      total_cache_creation_tokens = excluded.total_cache_creation_tokens,
      total_cache_read_tokens = excluded.total_cache_read_tokens,
      estimated_cost_usd = excluded.estimated_cost_usd,
      created_at = excluded.created_at,
      modified_at = excluded.modified_at,
      synced_at = datetime('now')
  `);

  const insertMessage = db.prepare(`
    INSERT INTO messages (
      uuid, session_id, parent_uuid, type, role,
      is_sidechain, agent_id, model, content_text, content_json,
      has_tool_use, has_thinking, tool_names,
      input_tokens, output_tokens, stop_reason,
      timestamp, line_number
    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
  `);

  const deleteSessionMessages = db.prepare(
    `DELETE FROM messages WHERE session_id = ?`
  );

  for (let i = 0; i < sessionFiles.length; i++) {
    const { projectEncoded, sessionId, filePath } = sessionFiles[i];

    if (i % 10 === 0) {
      onProgress?.({
        phase: "Parsing sessions",
        current: i,
        total: sessionFiles.length,
      });
    }

    const projectRow = getProjectId.get(projectEncoded) as
      | { id: number }
      | undefined;
    if (!projectRow) continue;

    let stat: fs.Stats;
    try {
      stat = fs.statSync(filePath);
    } catch {
      continue;
    }

    let result;
    try {
      result = await parseSessionFile(filePath);
    } catch (err) {
      console.error(`Error parsing ${filePath}:`, err);
      continue;
    }

    const { messages, metadata } = result;
    const primaryModel = getPrimaryModel(metadata.models);
    const cost = primaryModel
      ? estimateCost(
          primaryModel,
          metadata.totalInputTokens,
          metadata.totalOutputTokens,
          metadata.totalCacheCreationTokens,
          metadata.totalCacheReadTokens
        )
      : 0;

    db.exec("BEGIN TRANSACTION");
    try {
      // Upsert session
      insertSession.run(
        sessionId,
        projectRow.id,
        filePath,
        stat.mtimeMs,
        stat.size,
        metadata.firstPrompt,
        metadata.customTitle,
        messages.length,
        metadata.userMessageCount,
        metadata.assistantMessageCount,
        metadata.toolUseCount,
        metadata.gitBranch,
        metadata.cwd,
        primaryModel,
        metadata.version,
        metadata.permissionMode,
        0, // is_sidechain
        metadata.totalInputTokens,
        metadata.totalOutputTokens,
        metadata.totalCacheCreationTokens,
        metadata.totalCacheReadTokens,
        cost,
        metadata.createdAt,
        metadata.modifiedAt
      );

      // Delete old messages for this session (for re-sync)
      deleteSessionMessages.run(sessionId);

      // Insert messages in batches
      for (let j = 0; j < messages.length; j++) {
        const msg = messages[j];
        insertMessage.run(
          msg.uuid,
          sessionId,
          msg.parentUuid,
          msg.type,
          msg.role,
          msg.isSidechain ? 1 : 0,
          msg.agentId,
          msg.model,
          msg.contentText,
          msg.contentJson,
          msg.hasToolUse ? 1 : 0,
          msg.hasThinking ? 1 : 0,
          msg.toolNames.length > 0 ? msg.toolNames.join(",") : null,
          msg.inputTokens,
          msg.outputTokens,
          msg.stopReason,
          msg.timestamp,
          msg.lineNumber
        );
      }

      db.exec("COMMIT");
    } catch (err) {
      db.exec("ROLLBACK");
      console.error(`Error writing session ${sessionId}:`, err);
    }
  }

  // Phase 5: Parse subagents
  onProgress?.({ phase: "Parsing subagents", current: 0, total: 1 });
  const insertSubagent = db.prepare(`
    INSERT OR REPLACE INTO subagents (id, session_id, agent_type, description, file_path, message_count)
    VALUES (?, ?, ?, ?, ?, ?)
  `);

  db.exec("BEGIN TRANSACTION");
  for (const { projectEncoded, sessionId } of sessionFiles) {
    const subagentsDir = path.join(
      projectsDir,
      projectEncoded,
      sessionId,
      "subagents"
    );
    if (!fs.existsSync(subagentsDir)) continue;

    const metaFiles = fs
      .readdirSync(subagentsDir)
      .filter((f) => f.endsWith(".meta.json"));

    for (const metaFile of metaFiles) {
      const agentId = metaFile.replace(".meta.json", "").replace("agent-", "");
      const metaPath = path.join(subagentsDir, metaFile);
      const jsonlPath = path.join(
        subagentsDir,
        metaFile.replace(".meta.json", ".jsonl")
      );

      try {
        const meta: SubagentMeta = JSON.parse(
          fs.readFileSync(metaPath, "utf-8")
        );
        let lineCount = 0;
        if (fs.existsSync(jsonlPath)) {
          const content = fs.readFileSync(jsonlPath, "utf-8");
          lineCount = content.split("\n").filter((l) => l.trim()).length;
        }

        insertSubagent.run(
          agentId,
          sessionId,
          meta.agentType || null,
          meta.description || null,
          jsonlPath,
          lineCount
        );
      } catch {
        continue;
      }
    }
  }
  db.exec("COMMIT");

  // Phase 6: Check active sessions
  onProgress?.({ phase: "Checking active sessions", current: 0, total: 1 });
  const sessionsDir = path.join(claudeDir, "sessions");
  const markActive = db.prepare(
    `UPDATE sessions SET is_active = 1 WHERE id = ?`
  );
  const resetActive = db.prepare(`UPDATE sessions SET is_active = 0`);

  db.exec("BEGIN TRANSACTION");
  resetActive.run();

  if (fs.existsSync(sessionsDir)) {
    const sessionFiles2 = fs
      .readdirSync(sessionsDir)
      .filter((f) => f.endsWith(".json"));

    for (const file of sessionFiles2) {
      try {
        const data: ActiveSessionFile = JSON.parse(
          fs.readFileSync(path.join(sessionsDir, file), "utf-8")
        );
        if (data.sessionId) {
          markActive.run(data.sessionId);
        }
      } catch {
        continue;
      }
    }
  }
  db.exec("COMMIT");

  // Phase 7: Update project aggregates
  onProgress?.({ phase: "Updating project stats", current: 0, total: 1 });
  db.exec(`
    UPDATE projects SET
      session_count = (SELECT COUNT(*) FROM sessions WHERE project_id = projects.id),
      total_tokens = (SELECT COALESCE(SUM(total_input_tokens + total_output_tokens), 0) FROM sessions WHERE project_id = projects.id),
      last_activity_at = (SELECT MAX(modified_at) FROM sessions WHERE project_id = projects.id)
  `);

  // Save sync state
  const upsertState = db.prepare(
    `INSERT INTO sync_state (key, value) VALUES (?, ?)
     ON CONFLICT(key) DO UPDATE SET value = excluded.value`
  );
  upsertState.run("last_sync_at", new Date().toISOString());
  upsertState.run("session_count", String(sessionFiles.length));

  onProgress?.({
    phase: "Done",
    current: sessionFiles.length,
    total: sessionFiles.length,
  });
}

export async function syncSingleSession(filePath: string): Promise<void> {
  const db = getDb();
  const projectsDir = getProjectsDir();

  // Extract session ID and project from file path
  const fileName = path.basename(filePath, ".jsonl");
  const projectDir = path.dirname(filePath);
  const projectEncoded = path.basename(projectDir);

  // Check this is actually in the projects dir
  if (!projectDir.startsWith(projectsDir)) return;

  const projectRow = db
    .prepare(`SELECT id FROM projects WHERE encoded_name = ?`)
    .get(projectEncoded) as { id: number } | undefined;

  if (!projectRow) return;

  let stat: fs.Stats;
  try {
    stat = fs.statSync(filePath);
  } catch {
    return;
  }

  // Check if mtime changed
  const existing = db
    .prepare(`SELECT file_mtime FROM sessions WHERE id = ?`)
    .get(fileName) as { file_mtime: number } | undefined;

  if (existing && Math.abs(existing.file_mtime - stat.mtimeMs) < 100) {
    return; // No change
  }

  const result = await parseSessionFile(filePath);
  const { messages, metadata } = result;
  const primaryModel = getPrimaryModel(metadata.models);
  const cost = primaryModel
    ? estimateCost(
        primaryModel,
        metadata.totalInputTokens,
        metadata.totalOutputTokens,
        metadata.totalCacheCreationTokens,
        metadata.totalCacheReadTokens
      )
    : 0;

  db.exec("BEGIN TRANSACTION");
  try {
    db.prepare(`DELETE FROM messages WHERE session_id = ?`).run(fileName);

    db.prepare(`
      INSERT INTO sessions (
        id, project_id, file_path, file_mtime, file_size,
        first_prompt, custom_title, message_count,
        user_message_count, assistant_message_count, tool_use_count,
        git_branch, cwd, model, version, permission_mode,
        is_sidechain, total_input_tokens, total_output_tokens,
        total_cache_creation_tokens, total_cache_read_tokens,
        estimated_cost_usd, created_at, modified_at
      ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
      ON CONFLICT(id) DO UPDATE SET
        file_mtime = excluded.file_mtime,
        file_size = excluded.file_size,
        first_prompt = excluded.first_prompt,
        custom_title = excluded.custom_title,
        message_count = excluded.message_count,
        user_message_count = excluded.user_message_count,
        assistant_message_count = excluded.assistant_message_count,
        tool_use_count = excluded.tool_use_count,
        git_branch = excluded.git_branch,
        cwd = excluded.cwd,
        model = excluded.model,
        version = excluded.version,
        total_input_tokens = excluded.total_input_tokens,
        total_output_tokens = excluded.total_output_tokens,
        total_cache_creation_tokens = excluded.total_cache_creation_tokens,
        total_cache_read_tokens = excluded.total_cache_read_tokens,
        estimated_cost_usd = excluded.estimated_cost_usd,
        modified_at = excluded.modified_at,
        synced_at = datetime('now')
    `).run(
      fileName,
      projectRow.id,
      filePath,
      stat.mtimeMs,
      stat.size,
      metadata.firstPrompt,
      metadata.customTitle,
      messages.length,
      metadata.userMessageCount,
      metadata.assistantMessageCount,
      metadata.toolUseCount,
      metadata.gitBranch,
      metadata.cwd,
      primaryModel,
      metadata.version,
      metadata.permissionMode,
      0,
      metadata.totalInputTokens,
      metadata.totalOutputTokens,
      metadata.totalCacheCreationTokens,
      metadata.totalCacheReadTokens,
      cost,
      metadata.createdAt,
      metadata.modifiedAt
    );

    const insertMsg = db.prepare(`
      INSERT INTO messages (
        uuid, session_id, parent_uuid, type, role,
        is_sidechain, agent_id, model, content_text, content_json,
        has_tool_use, has_thinking, tool_names,
        input_tokens, output_tokens, stop_reason,
        timestamp, line_number
      ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    `);

    for (const msg of messages) {
      insertMsg.run(
        msg.uuid,
        fileName,
        msg.parentUuid,
        msg.type,
        msg.role,
        msg.isSidechain ? 1 : 0,
        msg.agentId,
        msg.model,
        msg.contentText,
        msg.contentJson,
        msg.hasToolUse ? 1 : 0,
        msg.hasThinking ? 1 : 0,
        msg.toolNames.length > 0 ? msg.toolNames.join(",") : null,
        msg.inputTokens,
        msg.outputTokens,
        msg.stopReason,
        msg.timestamp,
        msg.lineNumber
      );
    }

    db.exec("COMMIT");
  } catch (err) {
    db.exec("ROLLBACK");
    console.error(`Error syncing session ${fileName}:`, err);
  }

  // Update project aggregates
  db.exec(`
    UPDATE projects SET
      session_count = (SELECT COUNT(*) FROM sessions WHERE project_id = projects.id),
      total_tokens = (SELECT COALESCE(SUM(total_input_tokens + total_output_tokens), 0) FROM sessions WHERE project_id = projects.id),
      last_activity_at = (SELECT MAX(modified_at) FROM sessions WHERE project_id = projects.id)
    WHERE id = ${projectRow.id}
  `);
}

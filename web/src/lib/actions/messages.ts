"use server";

import { getDb } from "@/lib/db/connection";
import type { MessageRow } from "@/types/db";

interface GetMessagesParams {
  sessionId: string;
  offset?: number;
  limit?: number;
  excludeSidechain?: boolean;
}

export async function getSessionMessages(params: GetMessagesParams) {
  const db = getDb();
  const {
    sessionId,
    offset = 0,
    limit = 200,
    excludeSidechain = true,
  } = params;

  const conditions = ["session_id = ?"];
  const bindParams: unknown[] = [sessionId];

  if (excludeSidechain) {
    conditions.push("is_sidechain = 0");
  }

  // Only show user and assistant messages (not system/progress)
  conditions.push("type IN ('user', 'assistant')");

  const whereClause = `WHERE ${conditions.join(" AND ")}`;

  const total = db
    .prepare(`SELECT COUNT(*) as count FROM messages ${whereClause}`)
    .get(...bindParams) as { count: number };

  const messages = db
    .prepare(
      `SELECT * FROM messages ${whereClause}
       ORDER BY timestamp ASC, line_number ASC
       LIMIT ? OFFSET ?`
    )
    .all(...bindParams, limit, offset) as MessageRow[];

  return {
    messages,
    total: total.count,
    hasMore: offset + limit < total.count,
  };
}

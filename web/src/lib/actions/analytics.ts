"use server";

import { getDb } from "@/lib/db/connection";
import type { DashboardStats } from "@/types/db";

export async function getDashboardStats(): Promise<DashboardStats> {
  const db = getDb();

  const stats = db
    .prepare(
      `SELECT
        COUNT(*) as totalSessions,
        (SELECT COUNT(*) FROM projects) as totalProjects,
        (SELECT COUNT(*) FROM messages) as totalMessages,
        COALESCE(SUM(total_input_tokens), 0) as totalInputTokens,
        COALESCE(SUM(total_output_tokens), 0) as totalOutputTokens,
        COALESCE(SUM(estimated_cost_usd), 0) as estimatedTotalCost,
        SUM(CASE WHEN is_active = 1 THEN 1 ELSE 0 END) as activeSessions,
        SUM(CASE WHEN date(created_at) = date('now') THEN 1 ELSE 0 END) as todaySessions,
        COALESCE(AVG(message_count), 0) as avgMessageCount
       FROM sessions`
    )
    .get() as DashboardStats;

  return stats;
}

export async function getTokenUsageOverTime(params: {
  groupBy?: "day" | "week" | "month";
  dateFrom?: string;
  dateTo?: string;
  projectId?: number;
}) {
  const db = getDb();
  const { groupBy = "day", dateFrom, dateTo, projectId } = params;

  const dateFormat =
    groupBy === "month"
      ? "%Y-%m"
      : groupBy === "week"
        ? "%Y-W%W"
        : "%Y-%m-%d";

  const conditions: string[] = ["created_at IS NOT NULL"];
  const bindParams: unknown[] = [];

  if (dateFrom) {
    conditions.push("created_at >= ?");
    bindParams.push(dateFrom);
  }
  if (dateTo) {
    conditions.push("created_at <= ?");
    bindParams.push(dateTo);
  }
  if (projectId) {
    conditions.push("project_id = ?");
    bindParams.push(projectId);
  }

  const sql = `
    SELECT
      strftime('${dateFormat}', created_at) as date,
      SUM(total_input_tokens) as inputTokens,
      SUM(total_output_tokens) as outputTokens,
      SUM(estimated_cost_usd) as cost,
      COUNT(*) as sessionCount
    FROM sessions
    WHERE ${conditions.join(" AND ")}
    GROUP BY strftime('${dateFormat}', created_at)
    ORDER BY date ASC
  `;

  return db.prepare(sql).all(...bindParams) as {
    date: string;
    inputTokens: number;
    outputTokens: number;
    cost: number;
    sessionCount: number;
  }[];
}

export async function getProjectBreakdown() {
  const db = getDb();
  return db
    .prepare(
      `SELECT p.display_name as name, p.session_count as sessions,
              p.total_tokens as tokens
       FROM projects p
       WHERE p.session_count > 0
       ORDER BY p.session_count DESC
       LIMIT 15`
    )
    .all() as { name: string; sessions: number; tokens: number }[];
}

export async function getActivityData() {
  const db = getDb();
  return db
    .prepare(
      `SELECT date(created_at) as date, COUNT(*) as count
       FROM sessions
       WHERE created_at >= date('now', '-90 days')
       GROUP BY date(created_at)
       ORDER BY date ASC`
    )
    .all() as { date: string; count: number }[];
}

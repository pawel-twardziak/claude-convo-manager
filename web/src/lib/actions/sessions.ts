"use server";

import { getDb } from "@/lib/db/connection";
import type { SessionWithProject, FilterOptions } from "@/types/db";

interface GetSessionsParams {
  projectId?: number;
  gitBranch?: string;
  model?: string;
  dateFrom?: string;
  dateTo?: string;
  search?: string;
  sortBy?: string;
  sortDir?: "asc" | "desc";
  page?: number;
  pageSize?: number;
}

export async function getSessions(params: GetSessionsParams = {}) {
  const db = getDb();
  const {
    projectId,
    gitBranch,
    model,
    dateFrom,
    dateTo,
    search,
    sortBy = "modified_at",
    sortDir = "desc",
    page = 1,
    pageSize = 30,
  } = params;

  const conditions: string[] = [];
  const bindParams: unknown[] = [];

  if (projectId) {
    conditions.push("s.project_id = ?");
    bindParams.push(projectId);
  }
  if (gitBranch) {
    conditions.push("s.git_branch = ?");
    bindParams.push(gitBranch);
  }
  if (model) {
    conditions.push("s.model = ?");
    bindParams.push(model);
  }
  if (dateFrom) {
    conditions.push("s.created_at >= ?");
    bindParams.push(dateFrom);
  }
  if (dateTo) {
    conditions.push("s.created_at <= ?");
    bindParams.push(dateTo);
  }
  if (search) {
    conditions.push(
      "(s.first_prompt LIKE ? OR s.custom_title LIKE ?)"
    );
    const like = `%${search}%`;
    bindParams.push(like, like);
  }

  const whereClause =
    conditions.length > 0 ? `WHERE ${conditions.join(" AND ")}` : "";

  const allowedSorts = [
    "created_at",
    "modified_at",
    "message_count",
    "file_size",
    "total_input_tokens",
    "estimated_cost_usd",
  ];
  const safeSort = allowedSorts.includes(sortBy) ? sortBy : "modified_at";
  const safeDir = sortDir === "asc" ? "ASC" : "DESC";
  const offset = (page - 1) * pageSize;

  const countSql = `
    SELECT COUNT(*) as total
    FROM sessions s
    ${whereClause}
  `;

  const dataSql = `
    SELECT s.*, p.project_path, p.display_name as project_display_name
    FROM sessions s
    JOIN projects p ON p.id = s.project_id
    ${whereClause}
    ORDER BY s.${safeSort} ${safeDir}
    LIMIT ? OFFSET ?
  `;

  const { total } = db.prepare(countSql).get(...bindParams) as {
    total: number;
  };
  const sessions = db
    .prepare(dataSql)
    .all(...bindParams, pageSize, offset) as SessionWithProject[];

  return { sessions, total, page, pageSize };
}

export async function getSession(sessionId: string) {
  const db = getDb();
  return db
    .prepare(
      `SELECT s.*, p.project_path, p.display_name as project_display_name
       FROM sessions s
       JOIN projects p ON p.id = s.project_id
       WHERE s.id = ?`
    )
    .get(sessionId) as SessionWithProject | undefined;
}

export async function getFilterOptions(): Promise<FilterOptions> {
  const db = getDb();

  const projects = db
    .prepare(
      `SELECT id, display_name as displayName, session_count as sessionCount
       FROM projects
       ORDER BY session_count DESC`
    )
    .all() as FilterOptions["projects"];

  const branchRows = db
    .prepare(
      `SELECT DISTINCT git_branch FROM sessions
       WHERE git_branch IS NOT NULL AND git_branch != ''
       ORDER BY git_branch`
    )
    .all() as { git_branch: string }[];

  const modelRows = db
    .prepare(
      `SELECT DISTINCT model FROM sessions
       WHERE model IS NOT NULL
       ORDER BY model`
    )
    .all() as { model: string }[];

  const dateRange = db
    .prepare(
      `SELECT MIN(created_at) as min, MAX(created_at) as max FROM sessions`
    )
    .get() as { min: string; max: string };

  return {
    projects,
    branches: branchRows.map((r) => r.git_branch),
    models: modelRows.map((r) => r.model),
    dateRange,
  };
}

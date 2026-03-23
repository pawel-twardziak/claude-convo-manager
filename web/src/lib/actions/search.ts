"use server";

import { getDb } from "@/lib/db/connection";
import type { SearchResult } from "@/types/db";

interface SearchParams {
  query: string;
  projectId?: number;
  limit?: number;
  offset?: number;
}

function sanitizeFtsQuery(query: string): string {
  // Escape special FTS5 characters and wrap each word in quotes
  return query
    .replace(/['"]/g, "")
    .split(/\s+/)
    .filter((w) => w.length > 0)
    .map((word) => `"${word}"`)
    .join(" AND ");
}

export async function searchMessages(params: SearchParams) {
  const db = getDb();
  const { query, projectId, limit = 50, offset = 0 } = params;

  if (!query.trim()) return { results: [], total: 0 };

  const ftsQuery = sanitizeFtsQuery(query);
  if (!ftsQuery) return { results: [], total: 0 };

  const projectFilter = projectId
    ? "AND s.project_id = ?"
    : "";
  const bindParams: unknown[] = projectId
    ? [ftsQuery, projectId, limit, offset]
    : [ftsQuery, limit, offset];

  const countParams: unknown[] = projectId
    ? [ftsQuery, projectId]
    : [ftsQuery];

  const countSql = `
    SELECT COUNT(*) as total
    FROM messages_fts
    JOIN messages m ON m.id = messages_fts.rowid
    JOIN sessions s ON s.id = m.session_id
    WHERE messages_fts MATCH ?
    ${projectFilter}
  `;

  const dataSql = `
    SELECT
      m.id as message_id,
      m.session_id,
      p.display_name as project_display_name,
      COALESCE(s.custom_title, s.first_prompt) as session_title,
      snippet(messages_fts, 0, '<mark>', '</mark>', '...', 40) as snippet,
      m.timestamp,
      rank
    FROM messages_fts
    JOIN messages m ON m.id = messages_fts.rowid
    JOIN sessions s ON s.id = m.session_id
    JOIN projects p ON p.id = s.project_id
    WHERE messages_fts MATCH ?
    ${projectFilter}
    ORDER BY rank
    LIMIT ? OFFSET ?
  `;

  try {
    const { total } = db.prepare(countSql).get(...countParams) as {
      total: number;
    };
    const results = db.prepare(dataSql).all(...bindParams) as SearchResult[];
    return { results, total };
  } catch {
    return { results: [], total: 0 };
  }
}

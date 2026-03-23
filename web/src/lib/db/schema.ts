import Database from "better-sqlite3";

export function initializeSchema(db: Database.Database): void {
  db.exec(`
    CREATE TABLE IF NOT EXISTS sync_state (
      key TEXT PRIMARY KEY,
      value TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS projects (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      encoded_name TEXT NOT NULL UNIQUE,
      project_path TEXT NOT NULL,
      display_name TEXT,
      session_count INTEGER DEFAULT 0,
      total_tokens INTEGER DEFAULT 0,
      last_activity_at TEXT,
      created_at TEXT DEFAULT (datetime('now'))
    );
    CREATE INDEX IF NOT EXISTS idx_projects_path ON projects(project_path);

    CREATE TABLE IF NOT EXISTS sessions (
      id TEXT PRIMARY KEY,
      project_id INTEGER NOT NULL REFERENCES projects(id),
      file_path TEXT NOT NULL,
      file_mtime REAL,
      file_size INTEGER,
      first_prompt TEXT,
      custom_title TEXT,
      message_count INTEGER DEFAULT 0,
      user_message_count INTEGER DEFAULT 0,
      assistant_message_count INTEGER DEFAULT 0,
      tool_use_count INTEGER DEFAULT 0,
      git_branch TEXT,
      cwd TEXT,
      model TEXT,
      version TEXT,
      permission_mode TEXT,
      is_sidechain INTEGER DEFAULT 0,
      is_active INTEGER DEFAULT 0,
      total_input_tokens INTEGER DEFAULT 0,
      total_output_tokens INTEGER DEFAULT 0,
      total_cache_creation_tokens INTEGER DEFAULT 0,
      total_cache_read_tokens INTEGER DEFAULT 0,
      estimated_cost_usd REAL DEFAULT 0,
      created_at TEXT,
      modified_at TEXT,
      synced_at TEXT DEFAULT (datetime('now'))
    );
    CREATE INDEX IF NOT EXISTS idx_sessions_project ON sessions(project_id);
    CREATE INDEX IF NOT EXISTS idx_sessions_created ON sessions(created_at DESC);
    CREATE INDEX IF NOT EXISTS idx_sessions_modified ON sessions(modified_at DESC);
    CREATE INDEX IF NOT EXISTS idx_sessions_model ON sessions(model);
    CREATE INDEX IF NOT EXISTS idx_sessions_branch ON sessions(git_branch);

    CREATE TABLE IF NOT EXISTS messages (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      uuid TEXT,
      session_id TEXT NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
      parent_uuid TEXT,
      type TEXT NOT NULL,
      role TEXT,
      is_sidechain INTEGER DEFAULT 0,
      agent_id TEXT,
      model TEXT,
      content_text TEXT,
      content_json TEXT,
      has_tool_use INTEGER DEFAULT 0,
      has_thinking INTEGER DEFAULT 0,
      tool_names TEXT,
      input_tokens INTEGER DEFAULT 0,
      output_tokens INTEGER DEFAULT 0,
      stop_reason TEXT,
      timestamp TEXT,
      line_number INTEGER
    );
    CREATE INDEX IF NOT EXISTS idx_messages_session ON messages(session_id);
    CREATE INDEX IF NOT EXISTS idx_messages_type ON messages(type);
    CREATE INDEX IF NOT EXISTS idx_messages_timestamp ON messages(timestamp);
    CREATE INDEX IF NOT EXISTS idx_messages_agent ON messages(agent_id);
    CREATE UNIQUE INDEX IF NOT EXISTS idx_messages_session_uuid ON messages(session_id, uuid);

    CREATE VIRTUAL TABLE IF NOT EXISTS messages_fts USING fts5(
      content_text,
      content='messages',
      content_rowid='id',
      tokenize='porter unicode61'
    );

    CREATE TRIGGER IF NOT EXISTS messages_ai AFTER INSERT ON messages BEGIN
      INSERT INTO messages_fts(rowid, content_text)
      VALUES (new.id, new.content_text);
    END;

    CREATE TRIGGER IF NOT EXISTS messages_ad AFTER DELETE ON messages BEGIN
      INSERT INTO messages_fts(messages_fts, rowid, content_text)
      VALUES ('delete', old.id, old.content_text);
    END;

    CREATE TRIGGER IF NOT EXISTS messages_au AFTER UPDATE ON messages BEGIN
      INSERT INTO messages_fts(messages_fts, rowid, content_text)
      VALUES ('delete', old.id, old.content_text);
      INSERT INTO messages_fts(rowid, content_text)
      VALUES (new.id, new.content_text);
    END;

    CREATE TABLE IF NOT EXISTS subagents (
      id TEXT PRIMARY KEY,
      session_id TEXT NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
      agent_type TEXT,
      description TEXT,
      file_path TEXT,
      message_count INTEGER DEFAULT 0,
      created_at TEXT
    );
    CREATE INDEX IF NOT EXISTS idx_subagents_session ON subagents(session_id);

    CREATE TABLE IF NOT EXISTS tags (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      name TEXT NOT NULL UNIQUE,
      color TEXT DEFAULT '#6366f1'
    );

    CREATE TABLE IF NOT EXISTS session_tags (
      session_id TEXT NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
      tag_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
      created_at TEXT DEFAULT (datetime('now')),
      PRIMARY KEY (session_id, tag_id)
    );

    CREATE TABLE IF NOT EXISTS session_metadata (
      session_id TEXT PRIMARY KEY REFERENCES sessions(id) ON DELETE CASCADE,
      is_favorite INTEGER DEFAULT 0,
      notes TEXT,
      updated_at TEXT DEFAULT (datetime('now'))
    );
  `);
}

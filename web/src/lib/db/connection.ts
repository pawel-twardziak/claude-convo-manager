import Database from "better-sqlite3";
import path from "path";
import { initializeSchema } from "./schema";

const DATABASE_PATH =
  process.env.DATABASE_PATH ||
  path.join(process.cwd(), "data", "ccm.db");

let db: Database.Database | null = null;

export function getDb(): Database.Database {
  if (!db) {
    db = new Database(DATABASE_PATH);
    db.pragma("journal_mode = WAL");
    db.pragma("foreign_keys = ON");
    db.pragma("busy_timeout = 5000");
    initializeSchema(db);
  }
  return db;
}

export function closeDb(): void {
  if (db) {
    db.close();
    db = null;
  }
}

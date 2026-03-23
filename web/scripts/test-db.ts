import Database from "better-sqlite3";

const db = new Database("data/ccm.db");

console.log("FTS test - searching 'typescript':");
const results = db
  .prepare(
    `SELECT m.session_id, substr(snippet(messages_fts, 0, '>>>', '<<<', '...', 30), 1, 120) as snip
     FROM messages_fts
     JOIN messages m ON m.id = messages_fts.rowid
     WHERE messages_fts MATCH 'typescript'
     LIMIT 5`
  )
  .all();
console.table(results);

console.log("\nFTS test - searching 'database migration':");
const results2 = db
  .prepare(
    `SELECT m.session_id, substr(snippet(messages_fts, 0, '>>>', '<<<', '...', 30), 1, 120) as snip
     FROM messages_fts
     JOIN messages m ON m.id = messages_fts.rowid
     WHERE messages_fts MATCH 'database migration'
     LIMIT 5`
  )
  .all();
console.table(results2);

console.log("\nSessions with 0 messages:");
const empty = db
  .prepare("SELECT COUNT(*) as count FROM sessions WHERE message_count = 0")
  .get() as { count: number };
console.log(`  ${empty.count} sessions with 0 messages`);

console.log("\nModel distribution:");
console.table(
  db
    .prepare(
      "SELECT model, COUNT(*) as count FROM sessions WHERE model IS NOT NULL GROUP BY model ORDER BY count DESC"
    )
    .all()
);

db.close();

import { fullSync } from "../src/lib/sync/sync-engine";
import { getDb, closeDb } from "../src/lib/db/connection";

async function main() {
  console.log("Starting initial sync...\n");
  const startTime = Date.now();

  try {
    // Ensure DB is initialized
    getDb();

    await fullSync((progress) => {
      const pct =
        progress.total > 0
          ? Math.round((progress.current / progress.total) * 100)
          : 0;
      process.stdout.write(
        `\r[${progress.phase}] ${progress.current}/${progress.total} (${pct}%)`
      );
    });

    const elapsed = ((Date.now() - startTime) / 1000).toFixed(1);
    console.log(`\n\nSync completed in ${elapsed}s`);

    // Print stats
    const db = getDb();
    const projects = db
      .prepare("SELECT COUNT(*) as count FROM projects")
      .get() as { count: number };
    const sessions = db
      .prepare("SELECT COUNT(*) as count FROM sessions")
      .get() as { count: number };
    const messages = db
      .prepare("SELECT COUNT(*) as count FROM messages")
      .get() as { count: number };
    const tokens = db
      .prepare(
        "SELECT COALESCE(SUM(total_input_tokens + total_output_tokens), 0) as total FROM sessions"
      )
      .get() as { total: number };
    const cost = db
      .prepare("SELECT COALESCE(SUM(estimated_cost_usd), 0) as total FROM sessions")
      .get() as { total: number };

    console.log(`\nStats:`);
    console.log(`  Projects:  ${projects.count}`);
    console.log(`  Sessions:  ${sessions.count}`);
    console.log(`  Messages:  ${messages.count}`);
    console.log(`  Tokens:    ${tokens.total.toLocaleString()}`);
    console.log(`  Est. cost: $${cost.total.toFixed(2)}`);
  } catch (error) {
    console.error("\nSync failed:", error);
    process.exit(1);
  } finally {
    closeDb();
  }
}

main();

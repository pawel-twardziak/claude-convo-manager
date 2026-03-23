export async function register() {
  if (process.env.NEXT_RUNTIME === "nodejs") {
    // Run initial sync if DB is empty
    const { getDb } = await import("@/lib/db/connection");
    const db = getDb();
    const state = db
      .prepare("SELECT value FROM sync_state WHERE key = 'last_sync_at'")
      .get() as { value: string } | undefined;

    if (!state) {
      console.log("[Init] Database empty, running initial sync...");
      const { fullSync } = await import("@/lib/sync/sync-engine");
      await fullSync((p) => {
        if (p.current % 50 === 0) {
          console.log(`[Init] ${p.phase}: ${p.current}/${p.total}`);
        }
      });
      console.log("[Init] Initial sync complete");
    }

    // Start file watcher
    const { startSyncDaemon } = await import("@/lib/sync/watcher");
    startSyncDaemon();
  }
}

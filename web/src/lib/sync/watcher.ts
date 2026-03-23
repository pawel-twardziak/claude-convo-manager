import chokidar, { type FSWatcher } from "chokidar";
import { getClaudeDir, getProjectsDir } from "./path-encoder";
import { syncSingleSession } from "./sync-engine";

let watcher: FSWatcher | null = null;

export function startSyncDaemon(): void {
  if (watcher) return;

  const claudeDir = getClaudeDir();
  const projectsDir = getProjectsDir();

  console.log("[Sync Daemon] Watching for changes in", projectsDir);

  watcher = chokidar.watch([`${projectsDir}/**/*.jsonl`], {
    ignoreInitial: true,
    awaitWriteFinish: {
      stabilityThreshold: 1000,
      pollInterval: 200,
    },
    // Ignore subagent files — only watch top-level session files
    ignored: [
      "**/subagents/**",
      "**/tool-results/**",
      `${claudeDir}/history.jsonl`,
    ],
    depth: 1,
  });

  watcher.on("change", async (filePath: string) => {
    console.log("[Sync Daemon] File changed:", filePath);
    try {
      await syncSingleSession(filePath);
    } catch (err) {
      console.error("[Sync Daemon] Error syncing:", err);
    }
  });

  watcher.on("add", async (filePath: string) => {
    console.log("[Sync Daemon] New file:", filePath);
    try {
      await syncSingleSession(filePath);
    } catch (err) {
      console.error("[Sync Daemon] Error syncing new file:", err);
    }
  });
}

export function stopSyncDaemon(): void {
  if (watcher) {
    watcher.close();
    watcher = null;
  }
}

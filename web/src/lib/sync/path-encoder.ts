import path from "path";
import os from "os";

export function encodeProjectPath(absPath: string): string {
  return absPath.replace(/[/.]/g, "-");
}

export function extractDisplayName(projectPath: string): string {
  return path.basename(projectPath) || projectPath;
}

export function getClaudeDir(): string {
  return process.env.CLAUDE_DIR || path.join(os.homedir(), ".claude");
}

export function getProjectsDir(): string {
  return path.join(getClaudeDir(), "projects");
}

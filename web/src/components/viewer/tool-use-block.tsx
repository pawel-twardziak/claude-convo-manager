"use client";

import { useState } from "react";

interface ToolBlock {
  type: string;
  name?: string;
  input?: Record<string, unknown>;
  id?: string;
}

const TOOL_ICONS: Record<string, string> = {
  Bash: "$",
  Read: "R",
  Write: "W",
  Edit: "E",
  Grep: "G",
  Glob: "*",
  Agent: "A",
};

export function ToolUseBlock({ block }: { block: ToolBlock }) {
  const [expanded, setExpanded] = useState(false);
  const toolName = block.name || "Unknown Tool";
  const icon = TOOL_ICONS[toolName] || "T";

  // Format input for display
  let inputDisplay = "";
  if (block.input) {
    if (block.input.command) {
      inputDisplay = String(block.input.command);
    } else if (block.input.file_path) {
      inputDisplay = String(block.input.file_path);
    } else if (block.input.pattern) {
      inputDisplay = String(block.input.pattern);
    } else if (block.input.prompt) {
      inputDisplay = String(block.input.prompt).slice(0, 100);
    }
  }

  return (
    <div className="border rounded-lg bg-muted/30 overflow-hidden text-sm">
      <button
        onClick={() => setExpanded(!expanded)}
        className="w-full flex items-center gap-2 px-3 py-2 hover:bg-muted/50 transition-colors text-left"
      >
        <span className="w-5 h-5 rounded bg-muted flex items-center justify-center text-[10px] font-mono font-bold shrink-0">
          {icon}
        </span>
        <span className="font-medium text-xs">{toolName}</span>
        {inputDisplay && (
          <span className="text-xs text-muted-foreground truncate flex-1">
            {inputDisplay}
          </span>
        )}
        <span className="text-xs text-muted-foreground shrink-0">
          {expanded ? "▼" : "▶"}
        </span>
      </button>
      {expanded && block.input && (
        <div className="border-t px-3 py-2 bg-muted/20">
          <pre className="text-xs overflow-x-auto whitespace-pre-wrap break-all">
            {JSON.stringify(block.input, null, 2)}
          </pre>
        </div>
      )}
    </div>
  );
}

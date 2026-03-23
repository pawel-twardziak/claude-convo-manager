"use client";

import { useState } from "react";
import { Button } from "@/components/ui/button";

type CopyState = "idle" | "copied-cmd" | "copied-id";

export function ResumeButton({ sessionId }: { sessionId: string }) {
  const [state, setState] = useState<CopyState>("idle");

  async function copy(text: string, feedback: CopyState, e: React.MouseEvent) {
    e.preventDefault();
    e.stopPropagation();
    await navigator.clipboard.writeText(text);
    setState(feedback);
    setTimeout(() => setState("idle"), 2000);
  }

  return (
    <span className="inline-flex gap-1">
      <Button
        variant="ghost"
        size="sm"
        className="h-7 px-2 text-[11px] cursor-pointer"
        onClick={(e) => copy(`claude --resume ${sessionId}`, "copied-cmd", e)}
        title="Copy: claude --resume <id>"
      >
        {state === "copied-cmd" ? "Copied!" : "Resume"}
      </Button>
      <Button
        variant="ghost"
        size="sm"
        className="h-7 px-2 text-[11px] font-mono cursor-pointer"
        onClick={(e) => copy(sessionId, "copied-id", e)}
        title="Copy session ID"
      >
        {state === "copied-id" ? "Copied!" : "ID"}
      </Button>
    </span>
  );
}

"use client";

import { useState } from "react";

export function CodeBlock({
  language,
  code,
}: {
  language: string;
  code: string;
}) {
  const [copied, setCopied] = useState(false);

  async function handleCopy() {
    await navigator.clipboard.writeText(code);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  }

  return (
    <div className="relative group my-2 rounded-md border bg-muted/50 overflow-hidden">
      <div className="flex items-center justify-between px-3 py-1.5 border-b bg-muted/80">
        <span className="text-[10px] text-muted-foreground font-mono">
          {language || "text"}
        </span>
        <button
          onClick={handleCopy}
          className="text-[10px] text-muted-foreground hover:text-foreground transition-colors"
        >
          {copied ? "Copied!" : "Copy"}
        </button>
      </div>
      <pre className="p-3 overflow-x-auto text-[13px] leading-relaxed">
        <code>{code}</code>
      </pre>
    </div>
  );
}

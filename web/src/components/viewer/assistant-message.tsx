"use client";

import { useState } from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { Badge } from "@/components/ui/badge";
import type { MessageRow } from "@/types/db";
import { CodeBlock } from "./code-block";
import { ToolUseBlock } from "./tool-use-block";

interface ContentBlock {
  type: string;
  text?: string;
  thinking?: string;
  name?: string;
  input?: Record<string, unknown>;
  id?: string;
}

export function AssistantMessage({ message }: { message: MessageRow }) {
  const [showThinking, setShowThinking] = useState(false);

  let blocks: ContentBlock[] = [];
  if (message.content_json) {
    try {
      blocks = JSON.parse(message.content_json);
    } catch {
      // Fallback to text
    }
  }

  const thinkingBlocks = blocks.filter((b) => b.type === "thinking" && b.thinking);
  const textBlocks = blocks.filter((b) => b.type === "text" && b.text);
  const toolBlocks = blocks.filter((b) => b.type === "tool_use");

  // If no parsed blocks, use content_text
  const hasBlocks = textBlocks.length > 0 || toolBlocks.length > 0;

  return (
    <div className="flex justify-start">
      <div className="max-w-[85%] space-y-2">
        {/* Thinking toggle */}
        {thinkingBlocks.length > 0 && (
          <div>
            <button
              onClick={() => setShowThinking(!showThinking)}
              className="text-xs text-muted-foreground hover:text-foreground transition-colors"
            >
              {showThinking ? "▼" : "▶"} Thinking...
            </button>
            {showThinking && (
              <div className="mt-1 p-3 rounded-md bg-muted/50 border text-xs text-muted-foreground whitespace-pre-wrap max-h-96 overflow-auto">
                {thinkingBlocks.map((b, i) => (
                  <span key={i}>{b.thinking}</span>
                ))}
              </div>
            )}
          </div>
        )}

        {/* Text content */}
        {hasBlocks ? (
          textBlocks.map((block, i) => (
            <div
              key={i}
              className="bg-card border rounded-2xl rounded-bl-md px-4 py-3 prose prose-sm dark:prose-invert max-w-none break-words [&_pre]:my-0 [&_pre]:p-0 [&_pre]:bg-transparent"
            >
              <ReactMarkdown
                remarkPlugins={[remarkGfm]}
                components={{
                  code({ className, children, ...props }) {
                    const match = /language-(\w+)/.exec(className || "");
                    const isInline = !match && !className;
                    if (isInline) {
                      return (
                        <code
                          className="bg-muted px-1 py-0.5 rounded text-[13px]"
                          {...props}
                        >
                          {children}
                        </code>
                      );
                    }
                    return (
                      <CodeBlock
                        language={match?.[1] || ""}
                        code={String(children).replace(/\n$/, "")}
                      />
                    );
                  },
                  pre({ children }) {
                    return <>{children}</>;
                  },
                }}
              >
                {block.text || ""}
              </ReactMarkdown>
            </div>
          ))
        ) : (
          message.content_text && (
            <div className="bg-card border rounded-2xl rounded-bl-md px-4 py-3">
              <p className="text-sm whitespace-pre-wrap">{message.content_text}</p>
            </div>
          )
        )}

        {/* Tool use blocks */}
        {toolBlocks.map((block, i) => (
          <ToolUseBlock key={i} block={block} />
        ))}

        {/* Message metadata */}
        <div className="flex items-center gap-2 px-1">
          {message.model && (
            <Badge variant="outline" className="text-[9px] py-0 font-normal">
              {message.model.replace("claude-", "").split("-").slice(0, 2).join("-")}
            </Badge>
          )}
          {(message.input_tokens > 0 || message.output_tokens > 0) && (
            <span className="text-[10px] text-muted-foreground">
              {message.input_tokens + message.output_tokens} tokens
            </span>
          )}
          {message.timestamp && (
            <span className="text-[10px] text-muted-foreground">
              {new Date(message.timestamp).toLocaleTimeString()}
            </span>
          )}
        </div>
      </div>
    </div>
  );
}

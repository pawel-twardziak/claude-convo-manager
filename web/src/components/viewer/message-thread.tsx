"use client";

import type { MessageRow } from "@/types/db";
import { UserMessage } from "./user-message";
import { AssistantMessage } from "./assistant-message";

export function MessageThread({ messages }: { messages: MessageRow[] }) {
  return (
    <div className="flex-1 overflow-auto">
      <div className="max-w-4xl mx-auto py-6 px-6 space-y-4">
        {messages.map((msg) => {
          if (msg.type === "user" && msg.role === "user") {
            return <UserMessage key={msg.id} message={msg} />;
          }
          if (msg.type === "assistant") {
            return <AssistantMessage key={msg.id} message={msg} />;
          }
          return null;
        })}
        {messages.length === 0 && (
          <div className="text-center text-sm text-muted-foreground py-8">
            No messages in this conversation.
          </div>
        )}
      </div>
    </div>
  );
}

import type { MessageRow } from "@/types/db";

export function UserMessage({ message }: { message: MessageRow }) {
  const isToolResult =
    message.content_json && !message.content_text?.startsWith("[Tool");

  return (
    <div className="flex justify-end">
      <div className="max-w-[85%] bg-primary text-primary-foreground rounded-2xl rounded-br-md px-4 py-3">
        <p className="text-sm whitespace-pre-wrap break-words">
          {message.content_text || (isToolResult ? "[Tool Result]" : "")}
        </p>
        {message.timestamp && (
          <p className="text-[10px] opacity-60 mt-1">
            {new Date(message.timestamp).toLocaleTimeString()}
          </p>
        )}
      </div>
    </div>
  );
}

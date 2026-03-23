import Link from "next/link";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import type { SessionWithProject } from "@/types/db";

function timeAgo(dateStr: string | null): string {
  if (!dateStr) return "";
  const diff = Date.now() - new Date(dateStr).getTime();
  const mins = Math.floor(diff / 60000);
  if (mins < 60) return `${mins}m ago`;
  const hours = Math.floor(mins / 60);
  if (hours < 24) return `${hours}h ago`;
  const days = Math.floor(hours / 24);
  return `${days}d ago`;
}

export function RecentSessions({
  sessions,
}: {
  sessions: SessionWithProject[];
}) {
  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-base">Recent Conversations</CardTitle>
      </CardHeader>
      <CardContent>
        <div className="space-y-1">
          {sessions.map((s) => (
            <Link
              key={s.id}
              href={`/conversations/${s.id}`}
              className="flex items-center gap-3 px-3 py-2 rounded-md hover:bg-accent transition-colors group"
            >
              <div className="flex-1 min-w-0">
                <p className="text-sm font-medium truncate">
                  {s.custom_title || s.first_prompt || s.id}
                </p>
                <p className="text-xs text-muted-foreground">
                  {s.project_display_name}
                </p>
              </div>
              <div className="flex items-center gap-2 shrink-0">
                {s.model && (
                  <Badge variant="secondary" className="text-[10px] font-normal">
                    {s.model.replace("claude-", "").split("-").slice(0, 2).join("-")}
                  </Badge>
                )}
                <span className="text-xs text-muted-foreground w-16 text-right">
                  {s.message_count} msgs
                </span>
                <span className="text-xs text-muted-foreground w-16 text-right">
                  {timeAgo(s.modified_at)}
                </span>
              </div>
            </Link>
          ))}
          {sessions.length === 0 && (
            <p className="text-sm text-muted-foreground py-4 text-center">
              No conversations found. Run the sync first.
            </p>
          )}
        </div>
      </CardContent>
    </Card>
  );
}

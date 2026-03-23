import Link from "next/link";
import { Badge } from "@/components/ui/badge";
import { ResumeButton } from "@/components/conversations/resume-button";
import type { SessionWithProject } from "@/types/db";

function formatTokens(n: number): string {
  if (n >= 1_000_000) return `${(n / 1_000_000).toFixed(1)}M`;
  if (n >= 1_000) return `${(n / 1_000).toFixed(1)}K`;
  return n.toString();
}

export function SessionHeader({ session }: { session: SessionWithProject }) {
  const totalTokens =
    session.total_input_tokens + session.total_output_tokens;

  return (
    <div className="border-b px-6 py-4 bg-card shrink-0">
      <div className="flex items-start justify-between gap-4">
        <div className="min-w-0">
          <h2 className="text-base font-semibold truncate">
            {session.custom_title || session.first_prompt || session.id}
          </h2>
          <div className="flex items-center gap-2 mt-1 text-xs text-muted-foreground flex-wrap">
            <Link
              href={`/conversations?project=${session.project_id}`}
              className="hover:underline"
            >
              {session.project_display_name}
            </Link>
            {session.git_branch && (
              <Badge variant="outline" className="text-[10px] py-0">
                {session.git_branch}
              </Badge>
            )}
            {session.model && (
              <Badge variant="secondary" className="text-[10px] py-0">
                {session.model.replace("claude-", "")}
              </Badge>
            )}
            <span>{session.message_count} messages</span>
            <span>{formatTokens(totalTokens)} tokens</span>
            <span>${session.estimated_cost_usd.toFixed(2)}</span>
            {session.created_at && (
              <span>
                {new Date(session.created_at).toLocaleDateString("en-US", {
                  year: "numeric",
                  month: "short",
                  day: "numeric",
                  hour: "2-digit",
                  minute: "2-digit",
                })}
              </span>
            )}
          </div>
        </div>
        <ResumeButton sessionId={session.id} />
      </div>
    </div>
  );
}

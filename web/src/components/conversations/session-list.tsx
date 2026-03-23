"use client";

import Link from "next/link";
import { useRouter, useSearchParams, usePathname } from "next/navigation";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { ResumeButton } from "./resume-button";
import type { SessionWithProject } from "@/types/db";

function formatDate(dateStr: string | null): string {
  if (!dateStr) return "-";
  return new Date(dateStr).toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  });
}

function formatTokens(n: number): string {
  if (n >= 1_000_000) return `${(n / 1_000_000).toFixed(1)}M`;
  if (n >= 1_000) return `${(n / 1_000).toFixed(1)}K`;
  return n.toString();
}

function formatSize(bytes: number | null): string {
  if (!bytes) return "-";
  if (bytes >= 1_048_576) return `${(bytes / 1_048_576).toFixed(1)} MB`;
  if (bytes >= 1024) return `${(bytes / 1024).toFixed(0)} KB`;
  return `${bytes} B`;
}

export function SessionList({
  sessions,
  total,
  page,
  pageSize,
}: {
  sessions: SessionWithProject[];
  total: number;
  page: number;
  pageSize: number;
}) {
  const router = useRouter();
  const pathname = usePathname();
  const searchParams = useSearchParams();
  const totalPages = Math.ceil(total / pageSize);

  function goToPage(p: number) {
    const params = new URLSearchParams(searchParams.toString());
    params.set("page", p.toString());
    router.push(`${pathname}?${params.toString()}`);
  }

  return (
    <div>
      <div className="text-xs text-muted-foreground mb-2">
        {total} conversation{total !== 1 ? "s" : ""}
      </div>
      <div className="border rounded-lg divide-y">
        {sessions.map((s) => (
          <div
            key={s.id}
            className="flex items-center gap-3 px-4 py-3 hover:bg-accent/50 transition-colors"
          >
            <Link
              href={`/conversations/${s.id}`}
              className="flex-1 min-w-0"
            >
              <p className="text-sm font-medium truncate">
                {s.custom_title || s.first_prompt || s.id}
              </p>
              <div className="flex items-center gap-2 mt-1">
                <span className="text-xs text-muted-foreground">
                  {s.project_display_name}
                </span>
                {s.git_branch && (
                  <Badge variant="outline" className="text-[10px] py-0">
                    {s.git_branch}
                  </Badge>
                )}
              </div>
            </Link>
            <div className="flex items-center gap-3 shrink-0 text-xs text-muted-foreground">
              {s.model && (
                <Badge variant="secondary" className="text-[10px] font-normal">
                  {s.model.replace("claude-", "").split("-").slice(0, 2).join("-")}
                </Badge>
              )}
              <span className="w-12 text-right" title="Messages">
                {s.message_count}
              </span>
              <span className="w-14 text-right" title="Tokens">
                {formatTokens(s.total_input_tokens + s.total_output_tokens)}
              </span>
              <span className="w-14 text-right" title="Cost">
                ${s.estimated_cost_usd.toFixed(2)}
              </span>
              <span className="w-24 text-right" title="Modified">
                {formatDate(s.modified_at)}
              </span>
              <ResumeButton sessionId={s.id} />
            </div>
          </div>
        ))}
        {sessions.length === 0 && (
          <div className="py-8 text-center text-sm text-muted-foreground">
            No conversations match your filters.
          </div>
        )}
      </div>
      {totalPages > 1 && (
        <div className="flex justify-center gap-2 mt-4">
          <Button
            variant="outline"
            size="sm"
            onClick={() => goToPage(page - 1)}
            disabled={page <= 1}
          >
            Previous
          </Button>
          <span className="text-sm self-center text-muted-foreground">
            Page {page} of {totalPages}
          </span>
          <Button
            variant="outline"
            size="sm"
            onClick={() => goToPage(page + 1)}
            disabled={page >= totalPages}
          >
            Next
          </Button>
        </div>
      )}
    </div>
  );
}

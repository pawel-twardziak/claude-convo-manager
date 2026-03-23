import Link from "next/link";
import { ResumeButton } from "@/components/conversations/resume-button";
import type { SearchResult } from "@/types/db";

export function SearchResults({ results }: { results: SearchResult[] }) {
  if (results.length === 0) return null;

  return (
    <div className="space-y-2">
      {results.map((r) => (
        <div
          key={`${r.session_id}-${r.message_id}`}
          className="border rounded-lg p-3 hover:bg-accent/50 transition-colors flex items-start gap-2"
        >
          <Link
            href={`/conversations/${r.session_id}`}
            className="flex-1 min-w-0"
          >
            <div className="flex items-center gap-2 mb-1">
              <span className="text-sm font-medium truncate">
                {r.session_title || r.session_id}
              </span>
              <span className="text-xs text-muted-foreground shrink-0">
                {r.project_display_name}
              </span>
            </div>
            <p
              className="text-xs text-muted-foreground line-clamp-2 [&_mark]:bg-yellow-200 [&_mark]:dark:bg-yellow-800 [&_mark]:rounded-sm [&_mark]:px-0.5"
              dangerouslySetInnerHTML={{ __html: r.snippet }}
            />
            {r.timestamp && (
              <p className="text-[10px] text-muted-foreground mt-1">
                {new Date(r.timestamp).toLocaleString()}
              </p>
            )}
          </Link>
          <div className="shrink-0 pt-0.5">
            <ResumeButton sessionId={r.session_id} />
          </div>
        </div>
      ))}
    </div>
  );
}

import { getSessions, getFilterOptions } from "@/lib/actions/sessions";
import { SessionList } from "@/components/conversations/session-list";
import { FilterPanel } from "@/components/conversations/filter-panel";

interface Props {
  searchParams: Promise<Record<string, string | undefined>>;
}

export default async function ConversationsPage({ searchParams }: Props) {
  const params = await searchParams;
  const page = parseInt(params.page || "1", 10);
  const pageSize = 30;

  const [result, filterOptions] = await Promise.all([
    getSessions({
      projectId: params.project ? parseInt(params.project, 10) : undefined,
      gitBranch: params.branch || undefined,
      model: params.model || undefined,
      search: params.q || undefined,
      sortBy: params.sort || "modified_at",
      sortDir: (params.dir as "asc" | "desc") || "desc",
      page,
      pageSize,
    }),
    getFilterOptions(),
  ]);

  return (
    <div className="p-6 space-y-4 max-w-7xl">
      <h2 className="text-xl font-semibold">Conversations</h2>
      <FilterPanel options={filterOptions} />
      <SessionList
        sessions={result.sessions}
        total={result.total}
        page={page}
        pageSize={pageSize}
      />
    </div>
  );
}

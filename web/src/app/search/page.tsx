import { searchMessages } from "@/lib/actions/search";
import { SearchForm } from "@/components/search/search-form";
import { SearchResults } from "@/components/search/search-results";

interface Props {
  searchParams: Promise<Record<string, string | undefined>>;
}

export default async function SearchPage({ searchParams }: Props) {
  const params = await searchParams;
  const query = params.q || "";

  const result = query
    ? await searchMessages({ query, limit: 50 })
    : { results: [], total: 0 };

  return (
    <div className="p-6 space-y-4 max-w-4xl">
      <h2 className="text-xl font-semibold">Search</h2>
      <SearchForm defaultValue={query} />
      {query && (
        <p className="text-sm text-muted-foreground">
          {result.total} result{result.total !== 1 ? "s" : ""} for &quot;{query}&quot;
        </p>
      )}
      <SearchResults results={result.results} />
    </div>
  );
}

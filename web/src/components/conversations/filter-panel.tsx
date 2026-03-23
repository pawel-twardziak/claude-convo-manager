"use client";

import { useRouter, useSearchParams, usePathname } from "next/navigation";
import { useCallback } from "react";
import { Input } from "@/components/ui/input";
import type { FilterOptions } from "@/types/db";

export function FilterPanel({ options }: { options: FilterOptions }) {
  const router = useRouter();
  const pathname = usePathname();
  const searchParams = useSearchParams();

  const updateParam = useCallback(
    (key: string, value: string) => {
      const params = new URLSearchParams(searchParams.toString());
      if (value) {
        params.set(key, value);
      } else {
        params.delete(key);
      }
      params.delete("page"); // Reset to page 1 on filter change
      router.push(`${pathname}?${params.toString()}`);
    },
    [router, pathname, searchParams]
  );

  return (
    <div className="flex flex-wrap gap-3 items-center">
      <Input
        placeholder="Search conversations..."
        defaultValue={searchParams.get("q") || ""}
        onChange={(e) => {
          // Debounce search
          const value = e.target.value;
          const timeout = setTimeout(() => updateParam("q", value), 300);
          return () => clearTimeout(timeout);
        }}
        className="w-64"
      />
      <select
        className="h-9 rounded-md border border-input bg-background px-3 text-sm"
        value={searchParams.get("project") || ""}
        onChange={(e) => updateParam("project", e.target.value)}
      >
        <option value="">All Projects</option>
        {options.projects.map((p) => (
          <option key={p.id} value={p.id}>
            {p.displayName} ({p.sessionCount})
          </option>
        ))}
      </select>
      <select
        className="h-9 rounded-md border border-input bg-background px-3 text-sm"
        value={searchParams.get("model") || ""}
        onChange={(e) => updateParam("model", e.target.value)}
      >
        <option value="">All Models</option>
        {options.models.map((m) => (
          <option key={m} value={m}>
            {m.replace("claude-", "")}
          </option>
        ))}
      </select>
      <select
        className="h-9 rounded-md border border-input bg-background px-3 text-sm"
        value={searchParams.get("sort") || "modified_at"}
        onChange={(e) => updateParam("sort", e.target.value)}
      >
        <option value="modified_at">Last Modified</option>
        <option value="created_at">Created</option>
        <option value="message_count">Messages</option>
        <option value="estimated_cost_usd">Cost</option>
        <option value="total_input_tokens">Tokens</option>
      </select>
    </div>
  );
}

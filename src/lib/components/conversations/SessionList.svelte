<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { formatDate, formatTokens } from '$lib/utils';
	import ResumeButton from './ResumeButton.svelte';
	import type { SessionWithProject } from '$lib/types/db';

	let {
		sessions,
		total,
		currentPage,
		pageSize
	}: {
		sessions: SessionWithProject[];
		total: number;
		currentPage: number;
		pageSize: number;
	} = $props();

	let totalPages = $derived(Math.ceil(total / pageSize));

	function shortModel(model: string | null): string {
		if (!model) return '';
		return model.replace('claude-', '').split('-').slice(0, 2).join('-');
	}

	function goToPage(p: number) {
		const params = new URLSearchParams(page.url.searchParams.toString());
		params.set('page', p.toString());
		goto(`${page.url.pathname}?${params.toString()}`);
	}
</script>

<div>
	<div class="text-xs text-muted-foreground mb-2">
		{total} conversation{total !== 1 ? 's' : ''}
	</div>
	<div class="border rounded-lg divide-y">
		{#each sessions as s (s.id)}
			<div class="flex items-center gap-3 px-4 py-3 hover:bg-accent/50 transition-colors">
				<a href="/conversations/{s.id}" class="flex-1 min-w-0">
					<p class="text-sm font-medium truncate">
						{s.custom_title || s.first_prompt || s.id}
					</p>
					<div class="flex items-center gap-2 mt-1">
						<span class="text-xs text-muted-foreground">
							{s.project_display_name}
						</span>
						{#if s.git_branch}
							<span class="inline-flex items-center rounded-full border px-2 py-0 text-[10px] text-muted-foreground">
								{s.git_branch}
							</span>
						{/if}
					</div>
				</a>
				<div class="flex items-center gap-3 shrink-0 text-xs text-muted-foreground">
					{#if s.model}
						<span class="inline-flex items-center rounded-full border bg-secondary px-2 py-0.5 text-[10px] font-normal">
							{shortModel(s.model)}
						</span>
					{/if}
					<span class="w-12 text-right" title="Messages">{s.message_count}</span>
					<span class="w-14 text-right" title="Tokens">{formatTokens(s.total_input_tokens + s.total_output_tokens)}</span>
					<span class="w-14 text-right" title="Cost">${s.estimated_cost_usd.toFixed(2)}</span>
					<span class="w-24 text-right" title="Modified">{formatDate(s.modified_at)}</span>
					<ResumeButton sessionId={s.id} />
				</div>
			</div>
		{/each}
		{#if sessions.length === 0}
			<div class="py-8 text-center text-sm text-muted-foreground">
				No conversations match your filters.
			</div>
		{/if}
	</div>
	{#if totalPages > 1}
		<div class="flex justify-center gap-2 mt-4">
			<button
				class="h-8 px-3 text-sm rounded-md border border-input bg-background hover:bg-accent disabled:opacity-50"
				onclick={() => goToPage(currentPage - 1)}
				disabled={currentPage <= 1}
			>
				Previous
			</button>
			<span class="text-sm self-center text-muted-foreground">
				Page {currentPage} of {totalPages}
			</span>
			<button
				class="h-8 px-3 text-sm rounded-md border border-input bg-background hover:bg-accent disabled:opacity-50"
				onclick={() => goToPage(currentPage + 1)}
				disabled={currentPage >= totalPages}
			>
				Next
			</button>
		</div>
	{/if}
</div>

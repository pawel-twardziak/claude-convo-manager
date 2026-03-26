<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { SvelteURLSearchParams } from 'svelte/reactivity';
	import { getProjects } from '$lib/api/projects';
	import { formatTokens, formatCost, timeAgo } from '$lib/utils';
	import type { ProjectWithStats } from '$lib/types/db';

	let projects: ProjectWithStats[] = $state([]);
	let total = $state(0);
	let currentPage = $state(1);
	let pageSize = $state(30);
	let loading = $state(true);

	let debounceTimer: ReturnType<typeof setTimeout>;
	let totalPages = $derived(Math.ceil(total / pageSize));

	async function loadData() {
		loading = true;
		try {
			const params = page.url.searchParams;
			const search = params.get('q') || undefined;
			const sortBy = params.get('sort') || 'last_activity_at';
			const pg = params.get('page') ? Number(params.get('page')) : 1;

			const result = await getProjects({ search, sortBy, page: pg, pageSize: 30 });
			projects = result.projects;
			total = result.total;
			currentPage = result.page;
			pageSize = result.pageSize;
		} catch (e) {
			console.error('Failed to load projects:', e);
		} finally {
			loading = false;
		}
	}

	function updateParam(key: string, value: string) {
		const params = new SvelteURLSearchParams(page.url.searchParams.toString());
		if (value) {
			params.set(key, value);
		} else {
			params.delete(key);
		}
		params.delete('page');
		goto(resolve(`/projects?${params.toString()}`), { keepFocus: true, noScroll: true });
	}

	function onSearchInput(e: Event) {
		const value = (e.target as HTMLInputElement).value;
		clearTimeout(debounceTimer);
		debounceTimer = setTimeout(() => updateParam('q', value), 300);
	}

	function goToPage(p: number) {
		const params = new SvelteURLSearchParams(page.url.searchParams.toString());
		params.set('page', p.toString());
		goto(resolve(`/projects?${params.toString()}`));
	}

	$effect(() => {
		page.url.searchParams.toString();
		if (page.url.pathname.startsWith('/projects')) {
			loadData();
		}
	});
</script>

<div class="w-full space-y-4 p-6">
	<h2 class="text-xl font-semibold">Projects</h2>

	<div class="flex flex-wrap items-center gap-3">
		<input
			type="text"
			placeholder="Search projects..."
			value={page.url.searchParams.get('q') || ''}
			oninput={onSearchInput}
			class="border-input bg-background h-9 w-64 rounded-md border px-3 text-sm"
		/>
		<select
			class="border-input bg-background h-9 rounded-md border px-3 text-sm"
			value={page.url.searchParams.get('sort') || 'last_activity_at'}
			onchange={(e) => updateParam('sort', (e.target as HTMLSelectElement).value)}
		>
			<option value="last_activity_at">Last Active</option>
			<option value="display_name">Name</option>
			<option value="session_count">Sessions</option>
			<option value="total_tokens">Tokens</option>
			<option value="estimated_cost_usd">Cost</option>
			<option value="created_at">Created</option>
		</select>
	</div>

	{#if loading && projects.length === 0}
		<div class="space-y-2">
			{#each Array(5) as _, i (i)}
				<div class="h-20 animate-pulse rounded-lg border"></div>
			{/each}
		</div>
	{:else}
		<div class="text-muted-foreground mb-2 text-xs">
			{total} project{total !== 1 ? 's' : ''}
		</div>
		<div class="divide-y rounded-lg border">
			{#each projects as p (p.id)}
				<a
					href={resolve('/projects/[id]', { id: String(p.id) })}
					class="hover:bg-accent/50 flex items-center gap-4 px-4 py-3 transition-colors"
				>
					<div class="min-w-0 flex-1">
						<p class="truncate text-sm font-medium">
							{p.displayName || p.encodedName}
						</p>
						<p class="text-muted-foreground mt-0.5 truncate text-xs">
							{p.projectPath}
						</p>
						<div class="mt-1.5 flex flex-wrap items-center gap-1.5">
							{#if p.distinctModels}
								{#each p.distinctModels.split(',').filter(Boolean) as model (model)}
									<span
										class="bg-secondary inline-flex items-center rounded-full border px-2 py-0 text-[10px] font-normal"
									>
										{model.replace('claude-', '').split('-').slice(0, 2).join('-')}
									</span>
								{/each}
							{/if}
							{#if p.distinctBranches}
								{#each p.distinctBranches.split(',').filter(Boolean).slice(0, 5) as branch (branch)}
									<span
										class="text-muted-foreground inline-flex items-center rounded-full border px-2 py-0 text-[10px]"
									>
										{branch}
									</span>
								{/each}
								{#if p.distinctBranches.split(',').filter(Boolean).length > 5}
									<span class="text-muted-foreground text-[10px]">
										+{p.distinctBranches.split(',').filter(Boolean).length - 5}
									</span>
								{/if}
							{/if}
						</div>
					</div>
					<div class="text-muted-foreground flex shrink-0 items-center gap-4 text-xs">
						<span class="w-16 text-right" title="Sessions">
							{p.sessionCount} session{p.sessionCount !== 1 ? 's' : ''}
						</span>
						<span class="w-14 text-right" title="Tokens">{formatTokens(p.totalTokens)}</span>
						<span class="w-14 text-right" title="Cost">{formatCost(p.estimatedCostUsd)}</span>
						<span class="w-20 text-right" title="Last active">{timeAgo(p.lastActivityAt)}</span>
					</div>
				</a>
			{/each}
			{#if projects.length === 0}
				<div class="text-muted-foreground py-8 text-center text-sm">No projects match your search.</div>
			{/if}
		</div>
		{#if totalPages > 1}
			<div class="mt-4 flex justify-center gap-2">
				<button
					class="border-input bg-background hover:bg-accent h-8 rounded-md border px-3 text-sm disabled:opacity-50"
					onclick={() => goToPage(currentPage - 1)}
					disabled={currentPage <= 1}
				>
					Previous
				</button>
				<span class="text-muted-foreground self-center text-sm">
					Page {currentPage} of {totalPages}
				</span>
				<button
					class="border-input bg-background hover:bg-accent h-8 rounded-md border px-3 text-sm disabled:opacity-50"
					onclick={() => goToPage(currentPage + 1)}
					disabled={currentPage >= totalPages}
				>
					Next
				</button>
			</div>
		{/if}
	{/if}
</div>

<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { resolve } from '$app/paths';
	import { SvelteURLSearchParams } from 'svelte/reactivity';
	import { getSessions, getFilterOptions } from '$lib/api/sessions';
	import { getProjects } from '$lib/api/projects';
	import SessionList from '$lib/components/conversations/SessionList.svelte';
	import ProjectFilterPanel from '$lib/components/projects/ProjectFilterPanel.svelte';
	import Breadcrumbs from '$lib/components/layout/Breadcrumbs.svelte';
	import { formatTokens, formatCost, formatDate, timeAgo } from '$lib/utils';
	import type { SessionWithProject, FilterOptions, ProjectWithStats } from '$lib/types/db';

	let project: ProjectWithStats | null = $state(null);
	let sessions: SessionWithProject[] = $state([]);
	let total = $state(0);
	let currentPage = $state(1);
	let pageSize = $state(30);
	let options: FilterOptions | null = $state(null);
	let loading = $state(true);

	let projectId = $derived(Number(page.params.id));

	function handlePageChange(p: number) {
		const params = new SvelteURLSearchParams(page.url.searchParams.toString());
		params.set('page', p.toString());
		goto(resolve(`/projects/[id]?${params.toString()}`, { id: String(projectId) }));
	}

	async function loadData() {
		loading = true;
		try {
			const params = page.url.searchParams;
			const model = params.get('model') || undefined;
			const search = params.get('q') || undefined;
			const sortBy = params.get('sort') || 'modified_at';
			const pg = params.get('page') ? Number(params.get('page')) : 1;

			const [sessionsResult, filterOptions] = await Promise.all([
				getSessions({ projectId, model, search, sortBy, page: pg, pageSize: 30 }),
				options ? Promise.resolve(options) : getFilterOptions()
			]);

			sessions = sessionsResult.sessions;
			total = sessionsResult.total;
			currentPage = sessionsResult.page;
			pageSize = sessionsResult.pageSize;
			options = filterOptions;

			if (!project) {
				const allProjects = await getProjects({ pageSize: 1000 });
				project = allProjects.projects.find((p) => p.id === projectId) ?? null;
			}
		} catch (e) {
			console.error('Failed to load project data:', e);
		} finally {
			loading = false;
		}
	}

	$effect(() => {
		page.url.searchParams.toString();
		page.params.id;
		if (page.url.pathname.startsWith('/projects/')) {
			loadData();
		}
	});
</script>

<div class="w-full space-y-4 p-6">
	<Breadcrumbs
		items={[
			{ label: 'Projects', href: resolve('/projects') },
			{ label: project?.displayName || project?.encodedName || 'Project' }
		]}
	/>

	{#if project}
		<div class="bg-card rounded-lg border p-4">
			<h2 class="text-xl font-semibold">{project.displayName || project.encodedName}</h2>
			<p class="text-muted-foreground mt-1 text-sm">{project.projectPath}</p>

			<div class="mt-4 flex flex-wrap gap-6 text-sm">
				<div>
					<span class="text-muted-foreground">Sessions</span>
					<p class="font-medium">{project.sessionCount}</p>
				</div>
				<div>
					<span class="text-muted-foreground">Total Tokens</span>
					<p class="font-medium">{formatTokens(project.totalTokens)}</p>
				</div>
				<div>
					<span class="text-muted-foreground">Input</span>
					<p class="font-medium">{formatTokens(project.totalInputTokens)}</p>
				</div>
				<div>
					<span class="text-muted-foreground">Output</span>
					<p class="font-medium">{formatTokens(project.totalOutputTokens)}</p>
				</div>
				<div>
					<span class="text-muted-foreground">Cache Create</span>
					<p class="font-medium">{formatTokens(project.totalCacheCreationTokens)}</p>
				</div>
				<div>
					<span class="text-muted-foreground">Cache Read</span>
					<p class="font-medium">{formatTokens(project.totalCacheReadTokens)}</p>
				</div>
				<div>
					<span class="text-muted-foreground">Cost</span>
					<p class="font-medium">{formatCost(project.estimatedCostUsd)}</p>
				</div>
				<div>
					<span class="text-muted-foreground">Last Active</span>
					<p class="font-medium">{timeAgo(project.lastActivityAt)}</p>
				</div>
				<div>
					<span class="text-muted-foreground">Created</span>
					<p class="font-medium">{formatDate(project.createdAt)}</p>
				</div>
			</div>

			{#if project.distinctModels || project.distinctBranches}
				<div class="mt-3 flex flex-wrap items-center gap-1.5">
					{#if project.distinctModels}
						{#each project.distinctModels.split(',').filter(Boolean) as model (model)}
							<span
								class="bg-secondary inline-flex items-center rounded-full border px-2 py-0.5 text-[10px] font-normal"
							>
								{model.replace('claude-', '').split('-').slice(0, 2).join('-')}
							</span>
						{/each}
					{/if}
					{#if project.distinctBranches}
						{#each project.distinctBranches.split(',').filter(Boolean) as branch (branch)}
							<span
								class="text-muted-foreground inline-flex items-center rounded-full border px-2 py-0 text-[10px]"
							>
								{branch}
							</span>
						{/each}
					{/if}
				</div>
			{/if}
		</div>
	{:else if loading}
		<div class="h-32 animate-pulse rounded-lg border"></div>
	{/if}

	<h3 class="text-lg font-semibold">Conversations</h3>

	{#if options}
		<ProjectFilterPanel {options} {projectId} />
	{/if}

	{#if loading && sessions.length === 0}
		<div class="space-y-2">
			{#each Array(5) as _, i (i)}
				<div class="h-16 animate-pulse rounded-lg border"></div>
			{/each}
		</div>
	{:else}
		<SessionList {sessions} {total} {currentPage} {pageSize} onPageChange={handlePageChange} />
	{/if}
</div>

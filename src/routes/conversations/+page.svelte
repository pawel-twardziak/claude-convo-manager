<script lang="ts">
	import { page } from '$app/state';
	import { getSessions, getFilterOptions } from '$lib/api/sessions';
	import { getSyncVersion } from '$lib/stores/sync.svelte';
	import FilterPanel from '$lib/components/conversations/FilterPanel.svelte';
	import SessionList from '$lib/components/conversations/SessionList.svelte';
	import type { SessionWithProject, FilterOptions } from '$lib/types/db';

	let sessions: SessionWithProject[] = $state([]);
	let total = $state(0);
	let currentPage = $state(1);
	let pageSize = $state(30);
	let options: FilterOptions | null = $state(null);
	let loading = $state(true);

	async function loadData() {
		loading = true;
		try {
			const params = page.url.searchParams;
			const projectId = params.get('project') ? Number(params.get('project')) : undefined;
			const model = params.get('model') || undefined;
			const search = params.get('q') || undefined;
			const sortBy = params.get('sort') || 'modified_at';
			const pg = params.get('page') ? Number(params.get('page')) : 1;

			const [sessionsResult, filterOptions] = await Promise.all([
				getSessions({ projectId, model, search, sortBy, page: pg, pageSize: 30 }),
				getFilterOptions()
			]);

			sessions = sessionsResult.sessions;
			total = sessionsResult.total;
			currentPage = sessionsResult.page;
			pageSize = sessionsResult.pageSize;
			options = filterOptions;
		} catch (e) {
			console.error('Failed to load conversations:', e);
		} finally {
			loading = false;
		}
	}

	// Re-fetch when URL params change or sync completes; skip if navigating away
	$effect(() => {
		page.url.searchParams.toString();
		getSyncVersion();
		if (page.url.pathname.startsWith('/conversations')) {
			loadData();
		}
	});
</script>

<div class="w-full space-y-4 p-6">
	<h2 class="text-xl font-semibold">Conversations</h2>

	{#if options}
		<FilterPanel {options} />
	{/if}

	{#if loading && sessions.length === 0}
		<div class="space-y-2">
			{#each Array(5) as _, i (i)}
				<div class="h-16 animate-pulse rounded-lg border"></div>
			{/each}
		</div>
	{:else}
		<SessionList {sessions} {total} {currentPage} {pageSize} onSessionDeleted={loadData} />
	{/if}
</div>

<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { searchMessages } from '$lib/api/search';
	import { getSyncVersion } from '$lib/stores/sync.svelte';
	import SearchForm from '$lib/components/search/SearchForm.svelte';
	import SearchResults from '$lib/components/search/SearchResults.svelte';
	import type { SearchResult } from '$lib/types/db';

	let results: SearchResult[] = $state([]);
	let total = $state(0);
	let loading = $state(false);
	let searched = $state(false);

	async function doSearch() {
		const q = page.url.searchParams.get('q');
		if (!q) {
			results = [];
			total = 0;
			searched = false;
			return;
		}

		loading = true;
		searched = true;
		try {
			const res = await searchMessages({ query: q });
			results = res.results;
			total = res.total;
		} catch (e) {
			console.error('Search failed:', e);
			results = [];
			total = 0;
		} finally {
			loading = false;
		}
	}

	onMount(() => doSearch());

	$effect(() => {
		page.url.searchParams.get('q');
		getSyncVersion();
		doSearch();
	});
</script>

<div class="w-full space-y-4 p-6">
	<h2 class="text-xl font-semibold">Search</h2>

	<SearchForm defaultValue={page.url.searchParams.get('q') || ''} />

	{#if loading}
		<div class="space-y-2">
			{#each Array(3) as _, i (i)}
				<div class="h-20 animate-pulse rounded-lg border"></div>
			{/each}
		</div>
	{:else if searched}
		<p class="text-muted-foreground text-sm">
			{total} result{total !== 1 ? 's' : ''} found
		</p>
		<SearchResults {results} />
		{#if results.length === 0}
			<p class="text-muted-foreground py-8 text-center text-sm">No results found. Try a different search term.</p>
		{/if}
	{/if}
</div>

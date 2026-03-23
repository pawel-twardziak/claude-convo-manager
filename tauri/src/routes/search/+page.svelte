<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { searchMessages } from '$lib/api/search';
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
		doSearch();
	});
</script>

<div class="p-6 space-y-4 max-w-4xl">
	<h2 class="text-xl font-semibold">Search</h2>

	<SearchForm defaultValue={page.url.searchParams.get('q') || ''} />

	{#if loading}
		<div class="space-y-2">
			{#each Array(3) as _}
				<div class="h-20 rounded-lg border animate-pulse"></div>
			{/each}
		</div>
	{:else if searched}
		<p class="text-sm text-muted-foreground">
			{total} result{total !== 1 ? 's' : ''} found
		</p>
		<SearchResults {results} />
		{#if results.length === 0}
			<p class="text-center text-sm text-muted-foreground py-8">
				No results found. Try a different search term.
			</p>
		{/if}
	{/if}
</div>

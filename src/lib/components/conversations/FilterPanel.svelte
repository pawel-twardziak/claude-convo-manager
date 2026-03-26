<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { SvelteURLSearchParams } from 'svelte/reactivity';
	import type { FilterOptions } from '$lib/types/db';

	let { options }: { options: FilterOptions } = $props();
	let debounceTimer: ReturnType<typeof setTimeout>;

	function updateParam(key: string, value: string) {
		const params = new SvelteURLSearchParams(page.url.searchParams.toString());
		if (value) {
			params.set(key, value);
		} else {
			params.delete(key);
		}
		params.delete('page');
		goto(resolve(`/conversations?${params.toString()}`), { keepFocus: true, noScroll: true });
	}

	function onSearchInput(e: Event) {
		const value = (e.target as HTMLInputElement).value;
		clearTimeout(debounceTimer);
		debounceTimer = setTimeout(() => updateParam('q', value), 300);
	}
</script>

<div class="flex flex-wrap items-center gap-3">
	<input
		type="text"
		placeholder="Search conversations..."
		value={page.url.searchParams.get('q') || ''}
		oninput={onSearchInput}
		class="border-input bg-background h-9 w-64 rounded-md border px-3 text-sm"
	/>
	<select
		class="border-input bg-background h-9 rounded-md border px-3 text-sm"
		value={page.url.searchParams.get('project') || ''}
		onchange={(e) => updateParam('project', (e.target as HTMLSelectElement).value)}
	>
		<option value="">All Projects</option>
		{#each options.projects as p (p.id)}
			<option value={String(p.id)}>{p.displayName} ({p.sessionCount})</option>
		{/each}
	</select>
	<select
		class="border-input bg-background h-9 rounded-md border px-3 text-sm"
		value={page.url.searchParams.get('model') || ''}
		onchange={(e) => updateParam('model', (e.target as HTMLSelectElement).value)}
	>
		<option value="">All Models</option>
		{#each options.models as m (m)}
			<option value={m}>{m.replace('claude-', '')}</option>
		{/each}
	</select>
	<select
		class="border-input bg-background h-9 rounded-md border px-3 text-sm"
		value={page.url.searchParams.get('sort') || 'modified_at'}
		onchange={(e) => updateParam('sort', (e.target as HTMLSelectElement).value)}
	>
		<option value="modified_at">Last Modified</option>
		<option value="created_at">Created</option>
		<option value="message_count">Messages</option>
		<option value="estimated_cost_usd">Cost</option>
		<option value="total_input_tokens">Tokens</option>
	</select>
</div>

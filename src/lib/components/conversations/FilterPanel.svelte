<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import type { FilterOptions } from '$lib/types/db';

	let { options }: { options: FilterOptions } = $props();
	let debounceTimer: ReturnType<typeof setTimeout>;

	function updateParam(key: string, value: string) {
		const params = new URLSearchParams(page.url.searchParams.toString());
		if (value) {
			params.set(key, value);
		} else {
			params.delete(key);
		}
		params.delete('page');
		goto(`${page.url.pathname}?${params.toString()}`);
	}

	function onSearchInput(e: Event) {
		const value = (e.target as HTMLInputElement).value;
		clearTimeout(debounceTimer);
		debounceTimer = setTimeout(() => updateParam('q', value), 300);
	}
</script>

<div class="flex flex-wrap gap-3 items-center">
	<input
		type="text"
		placeholder="Search conversations..."
		value={page.url.searchParams.get('q') || ''}
		oninput={onSearchInput}
		class="w-64 h-9 rounded-md border border-input bg-background px-3 text-sm"
	/>
	<select
		class="h-9 rounded-md border border-input bg-background px-3 text-sm"
		value={page.url.searchParams.get('project') || ''}
		onchange={(e) => updateParam('project', (e.target as HTMLSelectElement).value)}
	>
		<option value="">All Projects</option>
		{#each options.projects as p}
			<option value={String(p.id)}>{p.displayName} ({p.sessionCount})</option>
		{/each}
	</select>
	<select
		class="h-9 rounded-md border border-input bg-background px-3 text-sm"
		value={page.url.searchParams.get('model') || ''}
		onchange={(e) => updateParam('model', (e.target as HTMLSelectElement).value)}
	>
		<option value="">All Models</option>
		{#each options.models as m}
			<option value={m}>{m.replace('claude-', '')}</option>
		{/each}
	</select>
	<select
		class="h-9 rounded-md border border-input bg-background px-3 text-sm"
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

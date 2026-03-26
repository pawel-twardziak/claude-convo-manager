<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { SvelteURLSearchParams } from 'svelte/reactivity';
	import { formatDate, formatTokens } from '$lib/utils';
	import { renameSession } from '$lib/api/sessions';
	import ResumeButton from './ResumeButton.svelte';
	import OpenInButton from './OpenInButton.svelte';
	import CloneButton from './CloneButton.svelte';
	import type { SessionWithProject } from '$lib/types/db';

	let editingId = $state<string | null>(null);
	let editValue = $state('');

	let clickTimer: ReturnType<typeof setTimeout> | undefined;

	function startRename(s: SessionWithProject) {
		editingId = s.id;
		editValue = s.custom_title || s.first_prompt || '';
		setTimeout(() => {
			const input = document.getElementById(`rename-${s.id}`) as HTMLInputElement;
			input?.select();
		}, 0);
	}

	function handleTitleClick(e: MouseEvent, s: SessionWithProject) {
		if (e.metaKey || e.ctrlKey || e.shiftKey) return;
		e.preventDefault();
		e.stopPropagation();
		clearTimeout(clickTimer);
		clickTimer = setTimeout(() => {
			goto(resolve('/conversations/[sessionId]', { sessionId: s.id }));
		}, 300);
	}

	function handleTitleDblClick(e: MouseEvent, s: SessionWithProject) {
		e.preventDefault();
		e.stopPropagation();
		clearTimeout(clickTimer);
		startRename(s);
	}

	async function saveRename(s: SessionWithProject) {
		const trimmed = editValue.trim();
		if (!trimmed || trimmed === s.custom_title) {
			editingId = null;
			return;
		}
		try {
			await renameSession(s.id, trimmed);
			s.custom_title = trimmed;
		} catch (err) {
			console.error('Failed to rename:', err);
		}
		editingId = null;
	}

	function onRenameKeydown(e: KeyboardEvent, s: SessionWithProject) {
		if (e.key === 'Enter') {
			e.preventDefault();
			saveRename(s);
		} else if (e.key === 'Escape') {
			editingId = null;
		}
	}

	let {
		sessions,
		total,
		currentPage,
		pageSize,
		onPageChange
	}: {
		sessions: SessionWithProject[];
		total: number;
		currentPage: number;
		pageSize: number;
		onPageChange?: (page: number) => void;
	} = $props();

	let totalPages = $derived(Math.ceil(total / pageSize));

	function shortModel(model: string | null): string {
		if (!model) return '';
		return model.replace('claude-', '').split('-').slice(0, 2).join('-');
	}

	function goToPage(p: number) {
		if (onPageChange) {
			onPageChange(p);
			return;
		}
		const params = new SvelteURLSearchParams(page.url.searchParams.toString());
		params.set('page', p.toString());
		goto(resolve(`/conversations?${params.toString()}`));
	}
</script>

<div>
	<div class="text-muted-foreground mb-2 text-xs">
		{total} conversation{total !== 1 ? 's' : ''}
	</div>
	<div class="divide-y rounded-lg border">
		{#each sessions as s (s.id)}
			<div class="hover:bg-accent/50 flex items-center gap-3 px-4 py-3 transition-colors">
				<a href={resolve('/conversations/[sessionId]', { sessionId: s.id })} class="group/row min-w-0 flex-1">
					{#if editingId === s.id}
						<input
							id="rename-{s.id}"
							bind:value={editValue}
							onkeydown={(e) => onRenameKeydown(e, s)}
							onblur={() => saveRename(s)}
							onclick={(e) => e.preventDefault()}
							class="border-primary w-full border-b-2 bg-transparent px-0 py-0.5 text-sm font-medium outline-none"
						/>
					{:else}
						<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
						<p
							class="cursor-pointer truncate text-sm font-medium"
							onclick={(e) => handleTitleClick(e, s)}
							ondblclick={(e) => handleTitleDblClick(e, s)}
							onkeydown={(e) => {
								if (e.key === 'F2') {
									e.preventDefault();
									startRename(s);
								}
							}}
							title="Double-click to rename"
						>
							{s.custom_title || s.first_prompt || s.id}
						</p>
					{/if}
					<div class="mt-1 flex items-center gap-2">
						<span class="text-muted-foreground text-xs">
							{s.project_display_name}
						</span>
						{#if s.git_branch}
							<span
								class="text-muted-foreground inline-flex items-center rounded-full border px-2 py-0 text-[10px]"
							>
								{s.git_branch}
							</span>
						{/if}
					</div>
				</a>
				<div class="text-muted-foreground flex shrink-0 items-center gap-3 text-xs">
					{#if s.model}
						<span
							class="bg-secondary inline-flex items-center rounded-full border px-2 py-0.5 text-[10px] font-normal"
						>
							{shortModel(s.model)}
						</span>
					{/if}
					<span class="w-12 text-right" title="Messages">{s.message_count}</span>
					<span class="w-14 text-right" title="Tokens"
						>{formatTokens(
							s.total_input_tokens +
								s.total_output_tokens +
								s.total_cache_creation_tokens +
								s.total_cache_read_tokens
						)}</span
					>
					<span class="w-14 text-right" title="Cost">${s.estimated_cost_usd.toFixed(2)}</span>
					<span class="w-24 text-right" title="Modified">{formatDate(s.modified_at)}</span>
					<CloneButton sessionId={s.id} currentProjectId={s.project_id} />
					<OpenInButton sessionId={s.id} cwd={s.cwd} projectPath={s.project_path} />
					<ResumeButton sessionId={s.id} />
				</div>
			</div>
		{/each}
		{#if sessions.length === 0}
			<div class="text-muted-foreground py-8 text-center text-sm">No conversations match your filters.</div>
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
</div>

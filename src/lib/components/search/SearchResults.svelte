<script lang="ts">
	import { resolve } from '$app/paths';
	import ResumeButton from '$lib/components/conversations/ResumeButton.svelte';
	import type { SearchResult } from '$lib/types/db';

	let { results }: { results: SearchResult[] } = $props();
</script>

{#if results.length > 0}
	<div class="space-y-2">
		{#each results as r (`${r.session_id}-${r.message_id}`)}
			<div class="hover:bg-accent/50 flex items-start gap-2 rounded-lg border p-3 transition-colors">
				<a href={resolve('/conversations/[sessionId]', { sessionId: r.session_id })} class="min-w-0 flex-1">
					<div class="mb-1 flex items-center gap-2">
						<span class="truncate text-sm font-medium">
							{r.session_title || r.session_id}
						</span>
						<span class="text-muted-foreground shrink-0 text-xs">
							{r.project_display_name}
						</span>
					</div>
					<p
						class="text-muted-foreground line-clamp-2 text-xs [&_mark]:rounded-sm [&_mark]:bg-yellow-200 [&_mark]:px-0.5 [&_mark]:dark:bg-yellow-800"
					>
						<!-- eslint-disable-next-line svelte/no-at-html-tags -- trusted <mark> highlights from server-side search -->
						{@html r.snippet}
					</p>
					{#if r.timestamp}
						<p class="text-muted-foreground mt-1 text-[10px]">
							{new Date(r.timestamp).toLocaleString()}
						</p>
					{/if}
				</a>
				<div class="shrink-0 pt-0.5">
					<ResumeButton sessionId={r.session_id} />
				</div>
			</div>
		{/each}
	</div>
{/if}

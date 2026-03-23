<script lang="ts">
	import ResumeButton from '$lib/components/conversations/ResumeButton.svelte';
	import type { SearchResult } from '$lib/types/db';

	let { results }: { results: SearchResult[] } = $props();
</script>

{#if results.length > 0}
	<div class="space-y-2">
		{#each results as r (`${r.session_id}-${r.message_id}`)}
			<div class="border rounded-lg p-3 hover:bg-accent/50 transition-colors flex items-start gap-2">
				<a href="/conversations/{r.session_id}" class="flex-1 min-w-0">
					<div class="flex items-center gap-2 mb-1">
						<span class="text-sm font-medium truncate">
							{r.session_title || r.session_id}
						</span>
						<span class="text-xs text-muted-foreground shrink-0">
							{r.project_display_name}
						</span>
					</div>
					<p class="text-xs text-muted-foreground line-clamp-2 [&_mark]:bg-yellow-200 [&_mark]:dark:bg-yellow-800 [&_mark]:rounded-sm [&_mark]:px-0.5">
						{@html r.snippet}
					</p>
					{#if r.timestamp}
						<p class="text-[10px] text-muted-foreground mt-1">
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

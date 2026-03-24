<script lang="ts">
	import { resolve } from '$app/paths';
	import type { SessionWithProject } from '$lib/types/db';
	import { timeAgo } from '$lib/utils';

	let { sessions }: { sessions: SessionWithProject[] } = $props();

	function shortModel(model: string | null): string {
		if (!model) return '';
		return model.replace('claude-', '').split('-').slice(0, 2).join('-');
	}
</script>

<div class="bg-card text-card-foreground rounded-lg border shadow-sm">
	<div class="p-4 pb-2">
		<h3 class="text-base font-semibold">Recent Conversations</h3>
	</div>
	<div class="px-4 pb-4">
		<div class="space-y-1">
			{#each sessions as s (s.id)}
				<a
					href={resolve('/conversations/[sessionId]', { sessionId: s.id })}
					class="hover:bg-accent flex items-center gap-3 rounded-md px-3 py-2 transition-colors"
				>
					<div class="min-w-0 flex-1">
						<p class="truncate text-sm font-medium">
							{s.custom_title || s.first_prompt || s.id}
						</p>
						<p class="text-muted-foreground text-xs">
							{s.project_display_name}
						</p>
					</div>
					<div class="flex shrink-0 items-center gap-2">
						{#if s.model}
							<span
								class="text-muted-foreground inline-flex items-center rounded-full border px-2 py-0.5 text-[10px] font-normal"
							>
								{shortModel(s.model)}
							</span>
						{/if}
						<span class="text-muted-foreground w-16 text-right text-xs">
							{s.message_count} msgs
						</span>
						<span class="text-muted-foreground w-16 text-right text-xs">
							{timeAgo(s.modified_at)}
						</span>
					</div>
				</a>
			{/each}
			{#if sessions.length === 0}
				<p class="text-muted-foreground py-4 text-center text-sm">
					No conversations found. Run the sync first.
				</p>
			{/if}
		</div>
	</div>
</div>

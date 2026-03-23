<script lang="ts">
	import type { SessionWithProject } from '$lib/types/db';
	import { timeAgo } from '$lib/utils';

	let { sessions }: { sessions: SessionWithProject[] } = $props();

	function shortModel(model: string | null): string {
		if (!model) return '';
		return model.replace('claude-', '').split('-').slice(0, 2).join('-');
	}
</script>

<div class="rounded-lg border bg-card text-card-foreground shadow-sm">
	<div class="p-4 pb-2">
		<h3 class="text-base font-semibold">Recent Conversations</h3>
	</div>
	<div class="px-4 pb-4">
		<div class="space-y-1">
			{#each sessions as s (s.id)}
				<a
					href="/conversations/{s.id}"
					class="flex items-center gap-3 px-3 py-2 rounded-md hover:bg-accent transition-colors"
				>
					<div class="flex-1 min-w-0">
						<p class="text-sm font-medium truncate">
							{s.custom_title || s.first_prompt || s.id}
						</p>
						<p class="text-xs text-muted-foreground">
							{s.project_display_name}
						</p>
					</div>
					<div class="flex items-center gap-2 shrink-0">
						{#if s.model}
							<span class="inline-flex items-center rounded-full border px-2 py-0.5 text-[10px] font-normal text-muted-foreground">
								{shortModel(s.model)}
							</span>
						{/if}
						<span class="text-xs text-muted-foreground w-16 text-right">
							{s.message_count} msgs
						</span>
						<span class="text-xs text-muted-foreground w-16 text-right">
							{timeAgo(s.modified_at)}
						</span>
					</div>
				</a>
			{/each}
			{#if sessions.length === 0}
				<p class="text-sm text-muted-foreground py-4 text-center">
					No conversations found. Run the sync first.
				</p>
			{/if}
		</div>
	</div>
</div>

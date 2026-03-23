<script lang="ts">
	import { formatTokens, formatDate } from '$lib/utils';
	import ResumeButton from '$lib/components/conversations/ResumeButton.svelte';
	import type { SessionWithProject } from '$lib/types/db';

	let { session }: { session: SessionWithProject } = $props();
	let totalTokens = $derived(session.total_input_tokens + session.total_output_tokens);
</script>

<div class="border-b px-6 py-4 bg-card shrink-0">
	<div class="flex items-start justify-between gap-4">
		<div class="min-w-0">
			<h2 class="text-base font-semibold truncate">
				{session.custom_title || session.first_prompt || session.id}
			</h2>
			<div class="flex items-center gap-2 mt-1 text-xs text-muted-foreground flex-wrap">
				<a href="/conversations?project={session.project_id}" class="hover:underline">
					{session.project_display_name}
				</a>
				{#if session.git_branch}
					<span class="inline-flex items-center rounded-full border px-2 py-0 text-[10px]">
						{session.git_branch}
					</span>
				{/if}
				{#if session.model}
					<span class="inline-flex items-center rounded-full border bg-secondary px-2 py-0 text-[10px]">
						{session.model.replace('claude-', '')}
					</span>
				{/if}
				<span>{session.message_count} messages</span>
				<span>{formatTokens(totalTokens)} tokens</span>
				<span>${session.estimated_cost_usd.toFixed(2)}</span>
				{#if session.created_at}
					<span>{formatDate(session.created_at)}</span>
				{/if}
			</div>
		</div>
		<ResumeButton sessionId={session.id} />
	</div>
</div>

<script lang="ts">
	import { resolve } from '$app/paths';
	import { formatTokens, formatDate } from '$lib/utils';
	import ResumeButton from '$lib/components/conversations/ResumeButton.svelte';
	import OpenInButton from '$lib/components/conversations/OpenInButton.svelte';
	import CloneButton from '$lib/components/conversations/CloneButton.svelte';
	import InlineRename from '$lib/components/conversations/InlineRename.svelte';
	import type { SessionWithProject } from '$lib/types/db';

	let { session }: { session: SessionWithProject } = $props();
	let totalTokens = $derived(
		session.total_input_tokens +
			session.total_output_tokens +
			session.total_cache_creation_tokens +
			session.total_cache_read_tokens
	);
</script>

<div class="bg-card shrink-0 border-b px-6 py-4">
	<div class="flex items-start justify-between gap-4">
		<div class="min-w-0">
			<h2 class="truncate text-base font-semibold">
				<InlineRename
					sessionId={session.id}
					currentTitle={session.custom_title}
					fallbackTitle={session.first_prompt}
					onRenamed={(title) => {
						session.custom_title = title;
					}}
				/>
			</h2>
			<div class="text-muted-foreground mt-1 flex flex-wrap items-center gap-2 text-xs">
				<a href={resolve(`/conversations?project=${session.project_id}`)} class="hover:underline">
					{session.project_display_name}
				</a>
				{#if session.git_branch}
					<span class="inline-flex items-center rounded-full border px-2 py-0 text-[10px]">
						{session.git_branch}
					</span>
				{/if}
				{#if session.model}
					<span class="bg-secondary inline-flex items-center rounded-full border px-2 py-0 text-[10px]">
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
		<span class="inline-flex gap-1">
			<CloneButton sessionId={session.id} currentProjectId={session.project_id} />
			<OpenInButton sessionId={session.id} cwd={session.cwd} projectPath={session.project_path} />
			<ResumeButton sessionId={session.id} />
		</span>
	</div>
</div>

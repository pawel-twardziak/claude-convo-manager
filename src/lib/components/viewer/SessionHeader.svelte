<script lang="ts">
	import { resolve } from '$app/paths';
	import { cleanTitle, formatTokens, formatDate } from '$lib/utils';
	import { goto } from '$app/navigation';
	import ResumeButton from '$lib/components/conversations/ResumeButton.svelte';
	import OpenInButton from '$lib/components/conversations/OpenInButton.svelte';
	import CloneButton from '$lib/components/conversations/CloneButton.svelte';
	import DeleteButton from '$lib/components/conversations/DeleteButton.svelte';
	import InlineRename from '$lib/components/conversations/InlineRename.svelte';
	import GitBranch from 'lucide-svelte/icons/git-branch';
	import { getSession } from '$lib/api/sessions';
	import type { SessionWithProject } from '$lib/types/db';

	let { session }: { session: SessionWithProject } = $props();
	let totalTokens = $derived(
		session.total_input_tokens +
			session.total_output_tokens +
			session.total_cache_creation_tokens +
			session.total_cache_read_tokens
	);

	let parentExists = $state<boolean | null>(null);
	$effect(() => {
		const parentId = session.forked_from_session_id;
		if (!parentId) {
			parentExists = null;
			return;
		}
		parentExists = null;
		getSession(parentId)
			.then((s) => {
				parentExists = s !== null;
			})
			.catch(() => {
				parentExists = false;
			});
	});
</script>

<div class="bg-card shrink-0 border-b px-6 py-4">
	<div class="flex items-start justify-between gap-4">
		<div class="min-w-0">
			<h2 class="truncate text-base font-semibold">
				<InlineRename
					sessionId={session.id}
					currentTitle={session.custom_title}
					fallbackTitle={session.ai_title || cleanTitle(session.first_prompt)}
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
				{#if session.forked_from_session_id}
					<span
						class="bg-secondary inline-flex items-center gap-1 rounded-full border px-2 py-0 text-[10px]"
						title="This session is a fork"
					>
						<GitBranch size={10} />
						{#if parentExists}
							Forked from
							<a
								href={resolve('/conversations/[sessionId]', {
									sessionId: session.forked_from_session_id
								})}
								class="hover:underline">parent</a
							>
							at line {session.forked_at_line_number}
						{:else}
							Forked at line {session.forked_at_line_number} (parent removed)
						{/if}
					</span>
				{/if}
			</div>
		</div>
		<span class="inline-flex gap-1">
			<CloneButton sessionId={session.id} currentProjectId={session.project_id} />
			<OpenInButton sessionId={session.id} cwd={session.cwd} projectPath={session.project_path} />
			<ResumeButton sessionId={session.id} />
			<DeleteButton sessionId={session.id} onDeleted={() => goto(resolve('/conversations'))} />
		</span>
	</div>
</div>

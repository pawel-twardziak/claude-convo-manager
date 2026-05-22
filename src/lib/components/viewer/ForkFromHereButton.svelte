<script lang="ts">
	import { AlertDialog } from 'bits-ui';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { forkSessionFromLine } from '$lib/api/messages';
	import { getFilterOptions } from '$lib/api/sessions';
	import { startSync } from '$lib/stores/sync.svelte';
	import GitBranch from 'lucide-svelte/icons/git-branch';

	let {
		sessionId,
		lineNumber,
		currentProjectId
	}: {
		sessionId: string;
		lineNumber: number;
		currentProjectId: number;
	} = $props();

	let open = $state(false);
	let forking = $state(false);
	let projects = $state<{ id: number; displayName: string; sessionCount: number }[]>([]);
	let targetProjectId = $state<number | null>(null);
	let warning = $state('');

	async function loadProjects() {
		if (projects.length > 0) return;
		try {
			const options = await getFilterOptions();
			projects = options.projects;
		} catch (err) {
			console.error('Failed to load projects:', err);
		}
	}

	async function handleFork(e: Event) {
		e.preventDefault();
		e.stopPropagation();
		forking = true;
		warning = '';
		try {
			const target = targetProjectId ?? currentProjectId;
			const crossProject = target !== currentProjectId;
			const result = await forkSessionFromLine(sessionId, lineNumber, crossProject ? target : undefined);
			await startSync();
			open = false;
			if (result.warning === 'fork_point_mid_tool_use') {
				warning = 'Forked mid-tool-use — the new session may not resume cleanly in Claude Code.';
				setTimeout(() => {
					goto(resolve('/conversations/[sessionId]', { sessionId: result.newSessionId }));
				}, 1500);
			} else {
				goto(resolve('/conversations/[sessionId]', { sessionId: result.newSessionId }));
			}
		} catch (err) {
			console.error('Failed to fork session:', err);
			warning = `Failed: ${err}`;
		} finally {
			forking = false;
		}
	}
</script>

<AlertDialog.Root
	bind:open
	onOpenChange={(isOpen) => {
		if (isOpen) {
			loadProjects();
			targetProjectId = currentProjectId;
		}
	}}
>
	<AlertDialog.Trigger
		title="Fork: create a new session up to this message"
		class="text-muted-foreground hover:bg-accent hover:text-accent-foreground bg-background/80 inline-flex h-6 cursor-pointer items-center gap-1 rounded-md border px-2 text-[10px] shadow-sm backdrop-blur"
		onclick={(e: MouseEvent) => {
			e.stopPropagation();
		}}
	>
		<GitBranch size={11} />
		Fork from here
	</AlertDialog.Trigger>
	{#if open}
		<AlertDialog.Portal>
			<AlertDialog.Overlay class="fixed inset-0 z-50 bg-black/50" />
			<AlertDialog.Content
				class="bg-card fixed top-1/2 left-1/2 z-50 w-full max-w-md -translate-x-1/2 -translate-y-1/2 rounded-lg border p-6 shadow-lg"
			>
				<AlertDialog.Title class="text-lg font-semibold">Fork from this message?</AlertDialog.Title>
				<AlertDialog.Description class="text-muted-foreground mt-2 text-sm">
					Creates a new session containing this message and everything before it. The original is left
					untouched.
				</AlertDialog.Description>

				<label class="mt-4 block text-xs font-medium">
					Target project
					<select
						bind:value={targetProjectId}
						disabled={forking}
						class="border-input bg-background mt-1 h-8 w-full rounded-md border px-2 text-xs outline-none"
					>
						{#each projects as p (p.id)}
							<option value={p.id}>
								{p.displayName}{p.id === currentProjectId ? ' (current)' : ''}
							</option>
						{/each}
					</select>
				</label>

				{#if warning}
					<p class="text-destructive mt-3 text-xs">{warning}</p>
				{/if}

				<div class="mt-4 flex justify-end gap-2">
					<AlertDialog.Cancel
						class="border-input bg-background hover:bg-accent h-9 cursor-pointer rounded-md border px-4 text-sm"
					>
						Cancel
					</AlertDialog.Cancel>
					<AlertDialog.Action
						class="bg-primary text-primary-foreground hover:bg-primary/90 h-9 cursor-pointer rounded-md px-4 text-sm disabled:opacity-50"
						disabled={forking}
						onclick={handleFork}
					>
						{forking ? 'Forking...' : 'Create fork'}
					</AlertDialog.Action>
				</div>
			</AlertDialog.Content>
		</AlertDialog.Portal>
	{/if}
</AlertDialog.Root>

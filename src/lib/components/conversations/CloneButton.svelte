<script lang="ts">
	import { DropdownMenu } from 'bits-ui';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { cloneSession, getFilterOptions } from '$lib/api/sessions';
	import { startSync } from '$lib/stores/sync.svelte';

	let {
		sessionId,
		currentProjectId
	}: {
		sessionId: string;
		currentProjectId: number;
	} = $props();

	let open = $state(false);
	let feedback = $state('');
	let projects = $state<{ id: number; displayName: string; sessionCount: number }[]>([]);
	let loading = $state(false);
	let searchQuery = $state('');

	let filteredProjects = $derived(
		projects
			.filter((p) => p.id !== currentProjectId)
			.filter((p) => p.displayName.toLowerCase().includes(searchQuery.toLowerCase()))
			.sort((a, b) => a.displayName.localeCompare(b.displayName))
	);

	function showFeedback(msg: string) {
		feedback = msg;
		setTimeout(() => (feedback = ''), 2000);
	}

	async function loadProjects() {
		if (projects.length > 0) return;
		try {
			const options = await getFilterOptions();
			projects = options.projects;
		} catch (err) {
			console.error('Failed to load projects:', err);
		}
	}

	async function handleClone(targetProjectId: number, e: Event) {
		e.preventDefault();
		e.stopPropagation();
		open = false;
		loading = true;
		try {
			const newSessionId = await cloneSession(sessionId, targetProjectId);
			await startSync();
			showFeedback('Cloned!');
			setTimeout(() => {
				goto(resolve('/conversations/[sessionId]', { sessionId: newSessionId }));
			}, 500);
		} catch (err) {
			console.error('Failed to clone session:', err);
			showFeedback('Failed!');
		} finally {
			loading = false;
		}
	}
</script>

<DropdownMenu.Root
	bind:open
	onOpenChange={(isOpen) => {
		if (isOpen) loadProjects();
		else searchQuery = '';
	}}
>
	<DropdownMenu.Trigger
		class="text-muted-foreground hover:bg-accent hover:text-accent-foreground inline-flex h-7 cursor-pointer items-center gap-1 rounded-md px-2 text-[11px]"
		onclick={(e: MouseEvent) => {
			e.preventDefault();
			e.stopPropagation();
		}}
	>
		{#if feedback}
			{feedback}
		{:else if loading}
			Cloning...
		{:else}
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="12"
				height="12"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
				><rect width="14" height="14" x="8" y="8" rx="2" ry="2" /><path
					d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"
				/></svg
			>
			Clone
		{/if}
	</DropdownMenu.Trigger>
	<DropdownMenu.Portal>
		<DropdownMenu.Content
			class="bg-popover text-popover-foreground z-50 max-h-64 min-w-[200px] overflow-y-auto rounded-md border p-1 shadow-md"
			sideOffset={4}
			align="end"
		>
			<DropdownMenu.Group>
				<DropdownMenu.GroupHeading
					class="text-muted-foreground px-2 py-1.5 text-[10px] font-semibold tracking-wider uppercase"
				>
					Clone to project
				</DropdownMenu.GroupHeading>
				<input
					type="text"
					placeholder="Search projects..."
					bind:value={searchQuery}
					onkeydown={(e) => e.stopPropagation()}
					onpointerdown={(e) => e.stopPropagation()}
					class="border-input bg-background mx-1 mb-1 h-7 w-[calc(100%-8px)] rounded-sm border px-2 text-xs outline-none"
				/>
				{#each filteredProjects as project (project.id)}
					<DropdownMenu.Item
						class="hover:bg-accent hover:text-accent-foreground data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground relative flex cursor-pointer items-center rounded-sm px-2 py-1.5 text-xs outline-none select-none"
						onSelect={(e) => handleClone(project.id, e)}
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							width="12"
							height="12"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
							class="mr-2 shrink-0"
							><path
								d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"
							/></svg
						>
						{project.displayName}
					</DropdownMenu.Item>
				{/each}
				{#if filteredProjects.length === 0}
					<div class="text-muted-foreground px-2 py-2 text-xs">
						{searchQuery ? 'No matching projects' : 'No other projects'}
					</div>
				{/if}
			</DropdownMenu.Group>
		</DropdownMenu.Content>
	</DropdownMenu.Portal>
</DropdownMenu.Root>

<script lang="ts">
	import { DropdownMenu } from 'bits-ui';
	import { writeText } from '@tauri-apps/plugin-clipboard-manager';
	import { openInApp, openTerminal } from '$lib/api/ide';
	import { getDetectedIDEs, getDetectedTerminals, hasDetectedApps } from '$lib/stores/ide.svelte';

	let {
		sessionId,
		cwd,
		projectPath
	}: {
		sessionId: string;
		cwd: string | null;
		projectPath: string;
	} = $props();

	let open = $state(false);
	let feedback = $state('');

	let ides = $derived(getDetectedIDEs());
	let terminals = $derived(getDetectedTerminals());
	let effectivePath = $derived(cwd || projectPath);

	function showFeedback(msg: string) {
		feedback = msg;
		setTimeout(() => (feedback = ''), 2000);
	}

	async function handleOpenIDE(appId: string, e: MouseEvent) {
		e.preventDefault();
		e.stopPropagation();
		open = false;
		try {
			await openInApp(appId, effectivePath);
			await writeText(`claude --resume ${sessionId}`);
			showFeedback('Opened + Copied!');
		} catch (err) {
			console.error('Failed to open IDE:', err);
		}
	}

	async function handleOpenTerminal(terminalId: string, e: MouseEvent) {
		e.preventDefault();
		e.stopPropagation();
		open = false;
		try {
			await openTerminal(terminalId, effectivePath);
		} catch (err) {
			console.error('Failed to open terminal:', err);
		}
	}
</script>

{#if hasDetectedApps()}
	<DropdownMenu.Root bind:open>
		<DropdownMenu.Trigger
			class="h-7 px-2 text-[11px] rounded-md text-muted-foreground hover:bg-accent hover:text-accent-foreground cursor-pointer inline-flex items-center gap-1"
			onclick={(e: MouseEvent) => { e.preventDefault(); e.stopPropagation(); }}
		>
			{#if feedback}
				{feedback}
			{:else}
				<svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/><polyline points="15 3 21 3 21 9"/><line x1="10" x2="21" y1="14" y2="3"/></svg>
				Open
			{/if}
		</DropdownMenu.Trigger>
		<DropdownMenu.Portal>
			<DropdownMenu.Content
				class="z-50 min-w-[160px] rounded-md border bg-popover p-1 text-popover-foreground shadow-md"
				sideOffset={4}
				align="end"
			>
				{#if ides.length > 0}
					<DropdownMenu.Group>
						<DropdownMenu.GroupHeading class="px-2 py-1.5 text-[10px] font-semibold text-muted-foreground uppercase tracking-wider">
							IDEs
						</DropdownMenu.GroupHeading>
						{#each ides as ide (ide.id)}
							<DropdownMenu.Item
								class="relative flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-xs outline-none hover:bg-accent hover:text-accent-foreground data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground"
								onSelect={(e) => handleOpenIDE(ide.id, e as unknown as MouseEvent)}
							>
								<svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mr-2 shrink-0"><polyline points="16 18 22 12 16 6"/><polyline points="8 6 2 12 8 18"/></svg>
								{ide.name}
							</DropdownMenu.Item>
						{/each}
					</DropdownMenu.Group>
				{/if}
				{#if ides.length > 0 && terminals.length > 0}
					<DropdownMenu.Separator class="my-1 h-px bg-muted" />
				{/if}
				{#if terminals.length > 0}
					<DropdownMenu.Group>
						<DropdownMenu.GroupHeading class="px-2 py-1.5 text-[10px] font-semibold text-muted-foreground uppercase tracking-wider">
							Terminals
						</DropdownMenu.GroupHeading>
						{#each terminals as term (term.id)}
							<DropdownMenu.Item
								class="relative flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-xs outline-none hover:bg-accent hover:text-accent-foreground data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground"
								onSelect={(e) => handleOpenTerminal(term.id, e as unknown as MouseEvent)}
							>
								<svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mr-2 shrink-0"><polyline points="4 17 10 11 4 5"/><line x1="12" x2="20" y1="19" y2="19"/></svg>
								{term.name}
							</DropdownMenu.Item>
						{/each}
					</DropdownMenu.Group>
				{/if}
			</DropdownMenu.Content>
		</DropdownMenu.Portal>
	</DropdownMenu.Root>
{/if}

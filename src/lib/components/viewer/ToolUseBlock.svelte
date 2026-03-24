<script lang="ts">
	interface ToolBlock {
		type: string;
		name?: string;
		input?: Record<string, unknown>;
		id?: string;
	}

	const TOOL_ICONS: Record<string, string> = {
		Bash: '$',
		Read: 'R',
		Write: 'W',
		Edit: 'E',
		Grep: 'G',
		Glob: '*',
		Agent: 'A'
	};

	let { block }: { block: ToolBlock } = $props();
	let expanded = $state(false);

	let toolName = $derived(block.name || 'Unknown Tool');
	let icon = $derived(TOOL_ICONS[toolName] || 'T');

	let inputDisplay = $derived.by(() => {
		if (!block.input) return '';
		if (block.input.command) return String(block.input.command);
		if (block.input.file_path) return String(block.input.file_path);
		if (block.input.pattern) return String(block.input.pattern);
		if (block.input.prompt) return String(block.input.prompt).slice(0, 100);
		return '';
	});
</script>

<div class="bg-muted/30 overflow-hidden rounded-lg border text-sm">
	<button
		onclick={() => (expanded = !expanded)}
		class="hover:bg-muted/50 flex w-full cursor-pointer items-center gap-2 px-3 py-2 text-left transition-colors"
	>
		<span
			class="bg-muted flex h-5 w-5 shrink-0 items-center justify-center rounded font-mono text-[10px] font-bold"
		>
			{icon}
		</span>
		<span class="text-xs font-medium">{toolName}</span>
		{#if inputDisplay}
			<span class="text-muted-foreground flex-1 truncate text-xs">{inputDisplay}</span>
		{/if}
		<span class="text-muted-foreground shrink-0 text-xs">
			{expanded ? '▼' : '▶'}
		</span>
	</button>
	{#if expanded && block.input}
		<div class="bg-muted/20 border-t px-3 py-2">
			<pre class="overflow-x-auto text-xs break-all whitespace-pre-wrap">{JSON.stringify(
					block.input,
					null,
					2
				)}</pre>
		</div>
	{/if}
</div>

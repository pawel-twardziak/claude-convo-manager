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

<div class="border rounded-lg bg-muted/30 overflow-hidden text-sm">
	<button
		onclick={() => (expanded = !expanded)}
		class="w-full flex items-center gap-2 px-3 py-2 hover:bg-muted/50 transition-colors text-left cursor-pointer"
	>
		<span class="w-5 h-5 rounded bg-muted flex items-center justify-center text-[10px] font-mono font-bold shrink-0">
			{icon}
		</span>
		<span class="font-medium text-xs">{toolName}</span>
		{#if inputDisplay}
			<span class="text-xs text-muted-foreground truncate flex-1">{inputDisplay}</span>
		{/if}
		<span class="text-xs text-muted-foreground shrink-0">
			{expanded ? '▼' : '▶'}
		</span>
	</button>
	{#if expanded && block.input}
		<div class="border-t px-3 py-2 bg-muted/20">
			<pre class="text-xs overflow-x-auto whitespace-pre-wrap break-all">{JSON.stringify(block.input, null, 2)}</pre>
		</div>
	{/if}
</div>

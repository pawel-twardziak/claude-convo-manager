<script lang="ts">
	import type { MessageRow } from '$lib/types/db';
	import Markdown from './Markdown.svelte';
	import ToolUseBlock from './ToolUseBlock.svelte';

	interface ContentBlock {
		type: string;
		text?: string;
		thinking?: string;
		name?: string;
		input?: Record<string, unknown>;
		id?: string;
	}

	let { message }: { message: MessageRow } = $props();
	let showThinking = $state(false);

	let blocks: ContentBlock[] = $derived.by(() => {
		if (!message.content_json) return [];
		try {
			return JSON.parse(message.content_json);
		} catch {
			return [];
		}
	});

	let thinkingBlocks = $derived(blocks.filter((b) => b.type === 'thinking' && b.thinking));
	let textBlocks = $derived(blocks.filter((b) => b.type === 'text' && b.text));
	let toolBlocks = $derived(blocks.filter((b) => b.type === 'tool_use'));
	let hasBlocks = $derived(textBlocks.length > 0 || toolBlocks.length > 0);

	function shortModel(model: string | null): string {
		if (!model) return '';
		return model.replace('claude-', '').split('-').slice(0, 2).join('-');
	}
</script>

<div class="flex justify-start">
	<div class="max-w-[85%] space-y-2">
		{#if thinkingBlocks.length > 0}
			<div>
				<button
					onclick={() => (showThinking = !showThinking)}
					class="text-muted-foreground hover:text-foreground cursor-pointer text-xs transition-colors"
				>
					{showThinking ? '▼' : '▶'} Thinking...
				</button>
				{#if showThinking}
					<div
						class="bg-muted/50 text-muted-foreground mt-1 max-h-96 overflow-auto rounded-md border p-3 text-xs whitespace-pre-wrap"
					>
						{#each thinkingBlocks as b, i (i)}
							<span>{b.thinking}</span>
						{/each}
					</div>
				{/if}
			</div>
		{/if}

		{#if hasBlocks}
			{#each textBlocks as block, i (i)}
				<div class="bg-card rounded-2xl rounded-bl-md border px-4 py-3">
					<Markdown content={block.text || ''} />
				</div>
			{/each}
		{:else if message.content_text}
			<div class="bg-card rounded-2xl rounded-bl-md border px-4 py-3">
				<p class="text-sm whitespace-pre-wrap">{message.content_text}</p>
			</div>
		{/if}

		{#each toolBlocks as block (block.id)}
			<ToolUseBlock {block} />
		{/each}

		<div class="flex items-center gap-2 px-1">
			{#if message.model}
				<span
					class="text-muted-foreground inline-flex items-center rounded-full border px-2 py-0 text-[9px] font-normal"
				>
					{shortModel(message.model)}
				</span>
			{/if}
			{#if message.input_tokens > 0 || message.output_tokens > 0}
				<span class="text-muted-foreground text-[10px]">
					{message.input_tokens + message.output_tokens} tokens
				</span>
			{/if}
			{#if message.timestamp}
				<span class="text-muted-foreground text-[10px]">
					{new Date(message.timestamp).toLocaleTimeString()}
				</span>
			{/if}
		</div>
	</div>
</div>

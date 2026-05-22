<script lang="ts">
	import type { MessageRow } from '$lib/types/db';

	let { message }: { message: MessageRow } = $props();

	let isToolResult = $derived(message.source_tool_use_uuid !== null);
	let isInterrupted = $derived(message.tool_use_interrupted === 1);

	let isError = $derived.by(() => {
		if (!isToolResult || !message.content_json) return false;
		try {
			const blocks = JSON.parse(message.content_json);
			return Array.isArray(blocks) && blocks.some((b) => b?.is_error === true);
		} catch {
			return false;
		}
	});

	let expanded = $state(false);
	let shortId = $derived(message.source_tool_use_uuid?.slice(-8) ?? '');

	let badge = $derived.by(() => {
		if (isInterrupted) return { label: 'Interrupted', tone: 'destructive' as const };
		if (isError) return { label: 'Tool error', tone: 'destructive' as const };
		return { label: 'Tool result', tone: 'muted' as const };
	});
</script>

{#if !isToolResult}
	<div class="flex justify-end" data-line-number={message.line_number}>
		<div class="bg-primary text-primary-foreground max-w-[85%] rounded-2xl rounded-br-md px-4 py-3">
			<p class="text-sm break-words whitespace-pre-wrap">{message.content_text || ''}</p>
			{#if message.timestamp}
				<p class="mt-1 text-[10px] opacity-60">
					{new Date(message.timestamp).toLocaleTimeString()}
				</p>
			{/if}
		</div>
	</div>
{:else}
	<div class="flex justify-start" data-line-number={message.line_number}>
		<div class="max-w-[85%] space-y-1">
			<div
				class={[
					'overflow-hidden rounded-lg border text-sm',
					badge.tone === 'destructive' ? 'border-destructive/40 bg-destructive/5' : 'bg-muted/30'
				]}
			>
				<button
					onclick={() => (expanded = !expanded)}
					class="hover:bg-muted/50 flex w-full cursor-pointer items-center gap-2 px-3 py-2 text-left transition-colors"
				>
					<span
						class={[
							'inline-flex items-center rounded-full px-2 py-0.5 text-[10px] font-medium tracking-wide uppercase',
							badge.tone === 'destructive'
								? 'bg-destructive/15 text-destructive'
								: 'bg-muted text-muted-foreground'
						]}
					>
						{badge.label}
					</span>
					{#if shortId}
						<span class="text-muted-foreground font-mono text-[10px]">
							{shortId}
						</span>
					{/if}
					<span class="text-muted-foreground flex-1 truncate text-xs">
						{message.content_text?.split('\n')[0] || ''}
					</span>
					<span class="text-muted-foreground shrink-0 text-xs">
						{expanded ? '▼' : '▶'}
					</span>
				</button>
				{#if expanded}
					<div class="bg-muted/10 border-t px-3 py-2">
						<pre
							class="max-h-96 overflow-auto text-xs break-all whitespace-pre-wrap">{message.content_text ||
								'[empty]'}</pre>
					</div>
				{/if}
			</div>
			{#if message.timestamp}
				<p class="text-muted-foreground px-1 text-[10px]">
					{new Date(message.timestamp).toLocaleTimeString()}
				</p>
			{/if}
		</div>
	</div>
{/if}

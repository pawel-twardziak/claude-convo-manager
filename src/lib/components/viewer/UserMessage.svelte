<script lang="ts">
	import type { MessageRow } from '$lib/types/db';

	let { message }: { message: MessageRow } = $props();
	let isToolResult = $derived(message.content_json && !message.content_text?.startsWith('[Tool'));
</script>

<div class="flex justify-end" data-line-number={message.line_number}>
	<div class="bg-primary text-primary-foreground max-w-[85%] rounded-2xl rounded-br-md px-4 py-3">
		<p class="text-sm break-words whitespace-pre-wrap">
			{message.content_text || (isToolResult ? '[Tool Result]' : '')}
		</p>
		{#if message.timestamp}
			<p class="mt-1 text-[10px] opacity-60">
				{new Date(message.timestamp).toLocaleTimeString()}
			</p>
		{/if}
	</div>
</div>

<script lang="ts">
	import type { MessageRow } from '$lib/types/db';

	let { message }: { message: MessageRow } = $props();
	let isToolResult = $derived(message.content_json && !message.content_text?.startsWith('[Tool'));
</script>

<div class="flex justify-end">
	<div class="max-w-[85%] bg-primary text-primary-foreground rounded-2xl rounded-br-md px-4 py-3">
		<p class="text-sm whitespace-pre-wrap break-words">
			{message.content_text || (isToolResult ? '[Tool Result]' : '')}
		</p>
		{#if message.timestamp}
			<p class="text-[10px] opacity-60 mt-1">
				{new Date(message.timestamp).toLocaleTimeString()}
			</p>
		{/if}
	</div>
</div>

<script lang="ts">
	import type { MessageRow } from '$lib/types/db';
	import UserMessage from './UserMessage.svelte';
	import AssistantMessage from './AssistantMessage.svelte';

	let { messages }: { messages: MessageRow[] } = $props();
</script>

<div>
	<div class="max-w-4xl mx-auto py-6 px-6 space-y-4">
		{#each messages as msg (msg.id)}
			{#if msg.type === 'user' && msg.role === 'user'}
				<UserMessage message={msg} />
			{:else if msg.type === 'assistant'}
				<AssistantMessage message={msg} />
			{/if}
		{/each}
		{#if messages.length === 0}
			<div class="text-center text-sm text-muted-foreground py-8">
				No messages in this conversation.
			</div>
		{/if}
	</div>
</div>

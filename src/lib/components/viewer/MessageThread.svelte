<script lang="ts">
	import type { MessageRow } from '$lib/types/db';
	import UserMessage from './UserMessage.svelte';
	import AssistantMessage from './AssistantMessage.svelte';
	import DeleteLastMessageButton from './DeleteLastMessageButton.svelte';

	let {
		messages,
		sessionId,
		onMessageDeleted
	}: {
		messages: MessageRow[];
		sessionId: string;
		onMessageDeleted: () => void;
	} = $props();
</script>

<div>
	<div class="mx-auto max-w-4xl space-y-4 px-6 py-6">
		{#each messages as msg, i (msg.id)}
			{#if msg.type === 'user' && msg.role === 'user'}
				<UserMessage message={msg} />
			{:else if msg.type === 'assistant'}
				<AssistantMessage message={msg} />
			{/if}
			{#if i === messages.length - 1}
				<div class="flex justify-end pt-1">
					<DeleteLastMessageButton {sessionId} onDeleted={onMessageDeleted} />
				</div>
			{/if}
		{/each}
		{#if messages.length === 0}
			<div class="text-muted-foreground py-8 text-center text-sm">No messages in this conversation.</div>
		{/if}
	</div>
</div>

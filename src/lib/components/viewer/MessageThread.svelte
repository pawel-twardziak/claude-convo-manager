<script lang="ts">
	import type { MessageRow } from '$lib/types/db';
	import UserMessage from './UserMessage.svelte';
	import AssistantMessage from './AssistantMessage.svelte';
	import DeleteFromHereButton from './DeleteFromHereButton.svelte';
	import ForkFromHereButton from './ForkFromHereButton.svelte';

	let {
		messages,
		sessionId,
		projectId,
		onMessageDeleted
	}: {
		messages: MessageRow[];
		sessionId: string;
		projectId: number;
		onMessageDeleted: () => void;
	} = $props();
</script>

<div>
	<div class="mx-auto max-w-4xl space-y-4 px-6 py-6">
		{#each messages as msg (msg.id)}
			{@const isUser = msg.type === 'user' && msg.role === 'user'}
			{@const isAssistant = msg.type === 'assistant'}
			{#if isUser || isAssistant}
				<div class="group/msg relative">
					{#if isUser}
						<UserMessage message={msg} />
					{:else}
						<AssistantMessage message={msg} />
					{/if}
					{#if msg.line_number != null}
						<div
							class="pointer-events-none absolute top-1 right-1 flex gap-1 opacity-0 transition-opacity group-hover/msg:opacity-100 focus-within:opacity-100 has-[[data-state=open]]:opacity-100 [&>*]:pointer-events-auto"
						>
							<ForkFromHereButton {sessionId} lineNumber={msg.line_number} currentProjectId={projectId} />
							<DeleteFromHereButton
								{sessionId}
								lineNumber={msg.line_number}
								onDeleted={onMessageDeleted}
							/>
						</div>
					{/if}
				</div>
			{/if}
		{/each}
		{#if messages.length === 0}
			<div class="text-muted-foreground py-8 text-center text-sm">No messages in this conversation.</div>
		{/if}
	</div>
</div>

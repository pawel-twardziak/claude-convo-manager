<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { getSession } from '$lib/api/sessions';
	import { getSessionMessages } from '$lib/api/messages';
	import SessionHeader from '$lib/components/viewer/SessionHeader.svelte';
	import MessageThread from '$lib/components/viewer/MessageThread.svelte';
	import type { SessionWithProject, MessageRow } from '$lib/types/db';

	let session: SessionWithProject | null = $state(null);
	let messages: MessageRow[] = $state([]);
	let loading = $state(true);
	let error = $state('');

	onMount(async () => {
		const sessionId = page.params.sessionId;
		try {
			const [s, m] = await Promise.all([
				getSession(sessionId),
				getSessionMessages({ sessionId, limit: 500 })
			]);
			if (!s) {
				error = 'Session not found';
			} else {
				session = s;
				messages = m.messages;
			}
		} catch (e) {
			error = `Failed to load conversation: ${e}`;
		} finally {
			loading = false;
		}
	});
</script>

{#if loading}
	<div class="flex items-center justify-center h-full">
		<p class="text-sm text-muted-foreground">Loading conversation...</p>
	</div>
{:else if error}
	<div class="flex items-center justify-center h-full">
		<p class="text-sm text-destructive">{error}</p>
	</div>
{:else if session}
	<div class="flex flex-col h-full">
		<SessionHeader {session} />
		<MessageThread {messages} />
	</div>
{/if}

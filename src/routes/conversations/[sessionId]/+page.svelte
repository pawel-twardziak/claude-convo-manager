<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { getSession } from '$lib/api/sessions';
	import { getSessionMessages } from '$lib/api/messages';
	import SessionHeader from '$lib/components/viewer/SessionHeader.svelte';
	import MessageThread from '$lib/components/viewer/MessageThread.svelte';
	import ConversationSearch from '$lib/components/viewer/ConversationSearch.svelte';
	import { openSearch, closeSearch, getIsOpen } from '$lib/stores/conversationSearch.svelte';
	import type { SessionWithProject, MessageRow } from '$lib/types/db';

	let session: SessionWithProject | null = $state(null);
	let messages: MessageRow[] = $state([]);
	let loading = $state(true);
	let error = $state('');
	let threadContainer: HTMLDivElement | undefined = $state();

	let searchOpen = $derived(getIsOpen());

	function handleKeydown(e: KeyboardEvent) {
		if ((e.ctrlKey || e.metaKey) && e.key === 'f') {
			e.preventDefault();
			openSearch();
		}
		if (e.key === 'Escape' && searchOpen) {
			e.preventDefault();
			closeSearch(threadContainer);
		}
	}

	onMount(async () => {
		const sessionId = page.params.sessionId!;
		try {
			const [s, m] = await Promise.all([getSession(sessionId), getSessionMessages({ sessionId, limit: 500 })]);
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
	<div class="flex h-full items-center justify-center">
		<p class="text-muted-foreground text-sm">Loading conversation...</p>
	</div>
{:else if error}
	<div class="flex h-full items-center justify-center">
		<p class="text-destructive text-sm">{error}</p>
	</div>
{:else if session}
	<div class="flex h-full flex-col">
		<SessionHeader {session} />
		<div class="relative flex-1 overflow-hidden">
			{#if searchOpen}
				<ConversationSearch container={threadContainer} />
			{/if}
			<div bind:this={threadContainer} class="h-full overflow-auto">
				<MessageThread {messages} />
			</div>
		</div>
	</div>
{/if}

<svelte:window onkeydown={handleKeydown} />

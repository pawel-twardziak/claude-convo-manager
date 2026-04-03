<script lang="ts">
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { getSession } from '$lib/api/sessions';
	import { getSessionMessages } from '$lib/api/messages';
	import { getSyncVersion, startSync } from '$lib/stores/sync.svelte';
	import SessionHeader from '$lib/components/viewer/SessionHeader.svelte';
	import MessageThread from '$lib/components/viewer/MessageThread.svelte';
	import ConversationSearch from '$lib/components/viewer/ConversationSearch.svelte';
	import Breadcrumbs from '$lib/components/layout/Breadcrumbs.svelte';
	import { openSearch, closeSearch, getIsOpen, setReplaceMode } from '$lib/stores/conversationSearch.svelte';
	import { tick } from 'svelte';
	import type { SessionWithProject, MessageRow } from '$lib/types/db';

	let session: SessionWithProject | null = $state(null);
	let messages: MessageRow[] = $state([]);
	let loading = $state(true);
	let error = $state('');
	let threadContainer: HTMLDivElement | undefined = $state();

	let searchOpen = $derived(getIsOpen());

	async function scrollToBottom() {
		await tick();
		if (threadContainer) {
			threadContainer.scrollTop = threadContainer.scrollHeight;
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if ((e.ctrlKey || e.metaKey) && e.key === 'f') {
			e.preventDefault();
			openSearch();
		}
		if ((e.ctrlKey || e.metaKey) && e.key === 'h') {
			e.preventDefault();
			openSearch();
			setReplaceMode(true);
		}
		if (e.key === 'Escape' && searchOpen) {
			e.preventDefault();
			closeSearch(threadContainer);
		}
	}

	async function reloadMessages() {
		const sid = page.params.sessionId!;
		const m = await getSessionMessages({ sessionId: sid, limit: 500 });
		messages = m.messages;
	}

	async function handleMessageDeleted() {
		await reloadMessages();
		const s = await getSession(page.params.sessionId!);
		if (s) session = s;
		startSync();
		scrollToBottom();
	}

	async function loadConversation() {
		const sessionId = page.params.sessionId!;
		loading = true;
		error = '';
		try {
			const [s, m] = await Promise.all([getSession(sessionId), getSessionMessages({ sessionId, limit: 500 })]);
			if (!s) {
				error = 'Session not found';
			} else {
				session = s;
				messages = m.messages;
				scrollToBottom();
			}
		} catch (e) {
			error = `Failed to load conversation: ${e}`;
		} finally {
			loading = false;
		}
	}

	$effect(() => {
		page.params.sessionId;
		getSyncVersion();
		loadConversation();
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
		<div class="bg-card shrink-0 border-b px-6 pt-3">
			<Breadcrumbs
				items={[
					{ label: 'Conversations', href: resolve('/conversations') },
					...(session.project_id
						? [
								{
									label: session.project_display_name || 'Project',
									href: resolve('/projects/[id]', { id: String(session.project_id) })
								}
							]
						: []),
					{ label: session.custom_title || session.first_prompt || 'Conversation' }
				]}
			/>
		</div>
		<SessionHeader {session} />
		<div class="relative flex-1 overflow-hidden">
			{#if searchOpen}
				<ConversationSearch
					container={threadContainer}
					sessionId={page.params.sessionId!}
					onReplace={reloadMessages}
				/>
			{/if}
			<div bind:this={threadContainer} class="h-full overflow-auto">
				<MessageThread {messages} sessionId={page.params.sessionId!} onMessageDeleted={handleMessageDeleted} />
			</div>
		</div>
	</div>
{/if}

<svelte:window onkeydown={handleKeydown} />

<script lang="ts">
	import { AlertDialog } from 'bits-ui';
	import { deleteLastMessage } from '$lib/api/messages';
	import Trash2 from 'lucide-svelte/icons/trash-2';

	let {
		sessionId,
		onDeleted
	}: {
		sessionId: string;
		onDeleted: () => void;
	} = $props();

	let open = $state(false);
	let deleting = $state(false);
	let feedback = $state('');

	function showFeedback(msg: string) {
		feedback = msg;
		setTimeout(() => (feedback = ''), 2000);
	}

	async function handleDelete(e: Event) {
		e.preventDefault();
		e.stopPropagation();
		deleting = true;
		try {
			await deleteLastMessage(sessionId);
			open = false;
			showFeedback('Deleted!');
			onDeleted();
		} catch (err) {
			console.error('Failed to delete last message:', err);
			showFeedback('Failed!');
		} finally {
			deleting = false;
		}
	}
</script>

<AlertDialog.Root bind:open>
	<AlertDialog.Trigger
		class="text-muted-foreground hover:bg-destructive/10 hover:text-destructive inline-flex h-7 cursor-pointer items-center gap-1 rounded-md px-2 text-[11px]"
		onclick={(e: MouseEvent) => {
			e.stopPropagation();
		}}
	>
		{#if feedback}
			{feedback}
		{:else}
			<Trash2 size={12} />
			Delete message
		{/if}
	</AlertDialog.Trigger>
	<AlertDialog.Portal>
		<AlertDialog.Overlay class="fixed inset-0 z-50 bg-black/50" />
		<AlertDialog.Content
			class="bg-card fixed top-1/2 left-1/2 z-50 w-full max-w-md -translate-x-1/2 -translate-y-1/2 rounded-lg border p-6 shadow-lg"
		>
			<AlertDialog.Title class="text-lg font-semibold">Delete last message?</AlertDialog.Title>
			<AlertDialog.Description class="text-muted-foreground mt-2 text-sm">
				This will remove the last message from the conversation file. This action cannot be undone.
			</AlertDialog.Description>
			<div class="mt-4 flex justify-end gap-2">
				<AlertDialog.Cancel
					class="border-input bg-background hover:bg-accent h-9 cursor-pointer rounded-md border px-4 text-sm"
				>
					Cancel
				</AlertDialog.Cancel>
				<AlertDialog.Action
					class="bg-destructive text-destructive-foreground hover:bg-destructive/90 h-9 cursor-pointer rounded-md px-4 text-sm disabled:opacity-50"
					disabled={deleting}
					onclick={handleDelete}
				>
					{deleting ? 'Deleting...' : 'Delete'}
				</AlertDialog.Action>
			</div>
		</AlertDialog.Content>
	</AlertDialog.Portal>
</AlertDialog.Root>

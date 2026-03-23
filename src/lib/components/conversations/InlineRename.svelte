<script lang="ts">
	import { renameSession } from '$lib/api/sessions';

	let {
		sessionId,
		currentTitle,
		fallbackTitle,
		onRenamed
	}: {
		sessionId: string;
		currentTitle: string | null;
		fallbackTitle: string | null;
		onRenamed?: (newTitle: string) => void;
	} = $props();

	let editing = $state(false);
	let inputValue = $state('');
	let saving = $state(false);
	let inputEl: HTMLInputElement | undefined = $state();

	let displayTitle = $derived(currentTitle || fallbackTitle || sessionId);

	function startEditing(e: MouseEvent) {
		e.preventDefault();
		e.stopPropagation();
		inputValue = currentTitle || fallbackTitle || '';
		editing = true;
		// Focus after Svelte updates the DOM
		setTimeout(() => inputEl?.select(), 0);
	}

	async function save() {
		if (saving) return;
		const trimmed = inputValue.trim();
		if (!trimmed || trimmed === currentTitle) {
			editing = false;
			return;
		}
		saving = true;
		try {
			await renameSession(sessionId, trimmed);
			onRenamed?.(trimmed);
			editing = false;
		} catch (e) {
			console.error('Failed to rename:', e);
		} finally {
			saving = false;
		}
	}

	function onKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			e.preventDefault();
			save();
		} else if (e.key === 'Escape') {
			editing = false;
		}
	}
</script>

{#if editing}
	<input
		bind:this={inputEl}
		bind:value={inputValue}
		onkeydown={onKeydown}
		onblur={save}
		disabled={saving}
		class="w-full text-base font-semibold bg-transparent border-b-2 border-primary outline-none px-0 py-0.5"
	/>
{:else}
	<span class="group inline-flex items-center gap-1.5 min-w-0">
		<span class="truncate">{displayTitle}</span>
		<button
			onclick={startEditing}
			class="shrink-0 opacity-0 group-hover:opacity-100 transition-opacity text-muted-foreground hover:text-foreground cursor-pointer"
			title="Rename conversation"
		>
			<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21.174 6.812a1 1 0 0 0-3.986-3.987L3.842 16.174a2 2 0 0 0-.5.83l-1.321 4.352a.5.5 0 0 0 .623.622l4.353-1.32a2 2 0 0 0 .83-.497z"/><path d="m15 5 4 4"/></svg>
		</button>
	</span>
{/if}

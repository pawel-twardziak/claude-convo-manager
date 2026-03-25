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

	function startEditing(e?: MouseEvent) {
		e?.preventDefault();
		e?.stopPropagation();
		inputValue = currentTitle || fallbackTitle || '';
		editing = true;
		// Focus after Svelte updates the DOM
		setTimeout(() => inputEl?.select(), 0);
	}

	function onTitleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' || e.key === 'F2') {
			e.preventDefault();
			startEditing();
		}
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
		class="border-primary w-full border-b-2 bg-transparent px-0 py-0.5 text-base font-semibold outline-none"
	/>
{:else}
	<span
		class="cursor-pointer truncate"
		ondblclick={startEditing}
		onkeydown={onTitleKeydown}
		tabindex="0"
		role="button"
		title="Double-click to rename"
	>
		{displayTitle}
	</span>
{/if}

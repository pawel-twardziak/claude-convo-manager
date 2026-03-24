<script lang="ts">
	import { onMount } from 'svelte';
	import {
		performHighlight,
		clearHighlights,
		closeSearch,
		nextMatch,
		prevMatch,
		getMatchLabel,
		getMatchCount
	} from '$lib/stores/conversationSearch.svelte';
	import ChevronUp from 'lucide-svelte/icons/chevron-up';
	import ChevronDown from 'lucide-svelte/icons/chevron-down';
	import X from 'lucide-svelte/icons/x';
	import Search from 'lucide-svelte/icons/search';

	let { container }: { container: HTMLElement | undefined } = $props();

	let inputEl: HTMLInputElement | undefined = $state();
	let localQuery = $state('');
	let debounceTimer: ReturnType<typeof setTimeout> | undefined;

	let matchLabel = $derived(getMatchLabel());
	let matchCount = $derived(getMatchCount());

	onMount(() => {
		inputEl?.focus();
		return () => {
			clearTimeout(debounceTimer);
			if (container) clearHighlights(container);
		};
	});

	function onInput() {
		clearTimeout(debounceTimer);
		debounceTimer = setTimeout(() => {
			if (container) performHighlight(container, localQuery);
		}, 150);
	}

	function onKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			e.preventDefault();
			close();
		} else if (e.key === 'Enter' && e.shiftKey) {
			e.preventDefault();
			prevMatch();
		} else if (e.key === 'Enter') {
			e.preventDefault();
			nextMatch();
		}
	}

	function close() {
		localQuery = '';
		closeSearch(container);
	}
</script>

<div
	class="absolute top-2 right-6 z-50 flex items-center gap-1 bg-card border rounded-md shadow-md px-2 py-1.5"
>
	<Search size={14} class="text-muted-foreground shrink-0" />
	<input
		bind:this={inputEl}
		bind:value={localQuery}
		oninput={onInput}
		onkeydown={onKeydown}
		placeholder="Search in conversation..."
		class="w-56 h-7 bg-transparent text-sm outline-none px-1"
	/>
	<span class="text-xs text-muted-foreground whitespace-nowrap min-w-16 text-center">
		{matchLabel}
	</span>
	<button
		onclick={prevMatch}
		disabled={matchCount === 0}
		class="p-1 rounded hover:bg-accent text-muted-foreground hover:text-foreground disabled:opacity-30 cursor-pointer disabled:cursor-default"
	>
		<ChevronUp size={14} />
	</button>
	<button
		onclick={nextMatch}
		disabled={matchCount === 0}
		class="p-1 rounded hover:bg-accent text-muted-foreground hover:text-foreground disabled:opacity-30 cursor-pointer disabled:cursor-default"
	>
		<ChevronDown size={14} />
	</button>
	<button
		onclick={close}
		class="p-1 rounded hover:bg-accent text-muted-foreground hover:text-foreground cursor-pointer"
	>
		<X size={14} />
	</button>
</div>

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

<div class="bg-card absolute top-2 right-6 z-50 flex items-center gap-1 rounded-md border px-2 py-1.5 shadow-md">
	<Search size={14} class="text-muted-foreground shrink-0" />
	<input
		bind:this={inputEl}
		bind:value={localQuery}
		oninput={onInput}
		onkeydown={onKeydown}
		placeholder="Search in conversation..."
		class="h-7 w-56 bg-transparent px-1 text-sm outline-none"
	/>
	<span class="text-muted-foreground min-w-16 text-center text-xs whitespace-nowrap">
		{matchLabel}
	</span>
	<button
		onclick={prevMatch}
		disabled={matchCount === 0}
		class="hover:bg-accent text-muted-foreground hover:text-foreground cursor-pointer rounded p-1 disabled:cursor-default disabled:opacity-30"
	>
		<ChevronUp size={14} />
	</button>
	<button
		onclick={nextMatch}
		disabled={matchCount === 0}
		class="hover:bg-accent text-muted-foreground hover:text-foreground cursor-pointer rounded p-1 disabled:cursor-default disabled:opacity-30"
	>
		<ChevronDown size={14} />
	</button>
	<button
		onclick={close}
		class="hover:bg-accent text-muted-foreground hover:text-foreground cursor-pointer rounded p-1"
	>
		<X size={14} />
	</button>
</div>

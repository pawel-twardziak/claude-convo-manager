<script lang="ts">
	import { onMount, tick } from 'svelte';
	import {
		performHighlight,
		clearHighlights,
		closeSearch,
		nextMatch,
		prevMatch,
		getMatchLabel,
		getMatchCount,
		getReplaceMode,
		toggleReplaceMode,
		getQuery,
		getCurrentMatchInfo,
		getIsReplacing,
		setIsReplacing
	} from '$lib/stores/conversationSearch.svelte';
	import { replaceInSession, replaceOneInSession } from '$lib/api/messages';
	import { startSync } from '$lib/stores/sync.svelte';
	import ChevronUp from 'lucide-svelte/icons/chevron-up';
	import ChevronDown from 'lucide-svelte/icons/chevron-down';
	import ChevronRight from 'lucide-svelte/icons/chevron-right';
	import X from 'lucide-svelte/icons/x';
	import Search from 'lucide-svelte/icons/search';

	let {
		container,
		sessionId,
		onReplace
	}: {
		container: HTMLElement | undefined;
		sessionId: string;
		onReplace: () => Promise<void>;
	} = $props();

	let inputEl: HTMLInputElement | undefined = $state();
	let localQuery = $state('');
	let replaceValue = $state('');
	let debounceTimer: ReturnType<typeof setTimeout> | undefined;

	let matchLabel = $derived(getMatchLabel());
	let matchCount = $derived(getMatchCount());
	let showReplace = $derived(getReplaceMode());
	let replacing = $derived(getIsReplacing());

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
		replaceValue = '';
		closeSearch(container);
	}

	let replaceError = $state('');

	async function handleReplaceOne() {
		const info = getCurrentMatchInfo();
		const searchTerm = getQuery();
		if (!info || !searchTerm || replacing) return;

		replaceError = '';
		setIsReplacing(true);
		try {
			const result = await replaceOneInSession({
				sessionId,
				searchTerm,
				replaceTerm: replaceValue,
				lineNumber: info.lineNumber,
				occurrenceIndex: info.occurrenceIndex
			});
			if (result.replacedCount === 0) {
				replaceError = 'No match in source file';
				return;
			}
			await startSync();
			await onReplace();
			await tick();
			if (container) performHighlight(container, localQuery);
		} catch (err) {
			console.error('Replace one failed:', err);
			replaceError = String(err);
		} finally {
			setIsReplacing(false);
		}
	}

	async function handleReplaceAll() {
		const searchTerm = getQuery();
		if (!searchTerm || matchCount === 0 || replacing) return;

		replaceError = '';
		setIsReplacing(true);
		try {
			const result = await replaceInSession({
				sessionId,
				searchTerm,
				replaceTerm: replaceValue
			});
			if (result.replacedCount === 0) {
				replaceError = 'No matches in source file';
				return;
			}
			await startSync();
			await onReplace();
			await tick();
			if (container) performHighlight(container, localQuery);
		} catch (err) {
			console.error('Replace all failed:', err);
			replaceError = String(err);
		} finally {
			setIsReplacing(false);
		}
	}
</script>

<div class="bg-card absolute top-2 right-6 z-50 flex flex-col gap-1 rounded-md border px-2 py-1.5 shadow-md">
	<div class="flex items-center gap-1">
		<button
			onclick={toggleReplaceMode}
			class="hover:bg-accent text-muted-foreground hover:text-foreground cursor-pointer rounded p-1 transition-transform"
			class:rotate-90={showReplace}
			title="Toggle Replace"
		>
			<ChevronRight size={14} />
		</button>
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

	{#if showReplace}
		<div class="flex items-center gap-1 pl-6">
			<input
				bind:value={replaceValue}
				placeholder="Replace with..."
				class="h-7 w-56 bg-transparent px-1 text-sm outline-none"
				onkeydown={(e) => {
					if (e.key === 'Escape') {
						e.preventDefault();
						close();
					}
				}}
			/>
			<button
				onclick={handleReplaceOne}
				disabled={matchCount === 0 || replacing}
				class="hover:bg-accent text-muted-foreground hover:text-foreground cursor-pointer rounded px-2 py-1 text-xs disabled:cursor-default disabled:opacity-30"
				title="Replace"
			>
				Replace
			</button>
			<button
				onclick={handleReplaceAll}
				disabled={matchCount === 0 || replacing}
				class="hover:bg-accent text-muted-foreground hover:text-foreground cursor-pointer rounded px-2 py-1 text-xs disabled:cursor-default disabled:opacity-30"
				title="Replace All"
			>
				All
			</button>
		</div>
		{#if replaceError}
			<div class="text-destructive pl-6 text-xs">{replaceError}</div>
		{/if}
	{/if}
</div>

<script lang="ts">
	import type { ResolvedPathname } from '$app/types';

	type BreadcrumbItem = {
		label: string;
		href?: ResolvedPathname;
	};

	let { items }: { items: BreadcrumbItem[] } = $props();
</script>

<nav aria-label="Breadcrumb" class="mb-2">
	<ol class="flex min-w-0 items-center gap-1.5 text-sm">
		<li class="shrink-0">
			<button
				onclick={() => history.back()}
				class="text-muted-foreground hover:text-foreground hover:bg-accent inline-flex h-6 w-6 items-center justify-center rounded transition-colors"
				title="Go back"
				aria-label="Go back"
			>
				&larr;
			</button>
		</li>
		<li class="text-muted-foreground shrink-0 select-none" aria-hidden="true">/</li>
		{#each items as item, i (i)}
			{#if i > 0}
				<li class="text-muted-foreground shrink-0 select-none" aria-hidden="true">/</li>
			{/if}
			<li class={i < items.length - 1 ? 'shrink-0' : 'min-w-0 overflow-hidden'}>
				{#if item.href && i < items.length - 1}
					<!-- eslint-disable-next-line svelte/no-navigation-without-resolve -- hrefs are pre-resolved by callers -->
					<a href={item.href} class="text-muted-foreground hover:text-foreground transition-colors">
						{item.label}
					</a>
				{:else}
					<span class="text-foreground block truncate font-medium">{item.label}</span>
				{/if}
			</li>
		{/each}
	</ol>
</nav>

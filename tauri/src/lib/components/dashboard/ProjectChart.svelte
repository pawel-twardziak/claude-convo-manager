<script lang="ts">
	interface ProjectData {
		name: string | null;
		sessions: number;
		tokens: number;
	}

	let { data }: { data: ProjectData[] } = $props();

	let maxSessions = $derived(Math.max(...data.map((d) => d.sessions), 1));
</script>

<div class="rounded-lg border bg-card text-card-foreground shadow-sm">
	<div class="p-4 pb-2">
		<h3 class="text-base font-semibold">Sessions by Project</h3>
	</div>
	<div class="px-4 pb-4">
		<div class="h-[300px] overflow-y-auto space-y-1.5">
			{#each data as item}
				<div class="flex items-center gap-2">
					<span class="text-[11px] text-muted-foreground w-[75px] truncate text-right shrink-0">
						{item.name || 'Unknown'}
					</span>
					<div class="flex-1 h-5 bg-muted rounded">
						<div
							class="h-full bg-primary rounded"
							style="width: {(item.sessions / maxSessions) * 100}%"
						></div>
					</div>
					<span class="text-xs text-muted-foreground w-8 text-right shrink-0">
						{item.sessions}
					</span>
				</div>
			{/each}
			{#if data.length === 0}
				<div class="h-full flex items-center justify-center text-sm text-muted-foreground">
					No project data available
				</div>
			{/if}
		</div>
	</div>
</div>

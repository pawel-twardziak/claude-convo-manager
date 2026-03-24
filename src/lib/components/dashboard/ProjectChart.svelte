<script lang="ts">
	interface ProjectData {
		name: string | null;
		sessions: number;
		tokens: number;
	}

	let { data }: { data: ProjectData[] } = $props();

	let maxSessions = $derived(Math.max(...data.map((d) => d.sessions), 1));
</script>

<div class="bg-card text-card-foreground rounded-lg border shadow-sm">
	<div class="p-4 pb-2">
		<h3 class="text-base font-semibold">Sessions by Project</h3>
	</div>
	<div class="px-4 pb-4">
		<div class="h-[300px] space-y-1.5 overflow-y-auto">
			{#each data as item (item.name)}
				<div class="flex items-center gap-2">
					<span class="text-muted-foreground w-[75px] shrink-0 truncate text-right text-[11px]">
						{item.name || 'Unknown'}
					</span>
					<div class="bg-muted h-5 flex-1 rounded">
						<div
							class="bg-primary h-full rounded"
							style="width: {(item.sessions / maxSessions) * 100}%"
						></div>
					</div>
					<span class="text-muted-foreground w-8 shrink-0 text-right text-xs">
						{item.sessions}
					</span>
				</div>
			{/each}
			{#if data.length === 0}
				<div class="text-muted-foreground flex h-full items-center justify-center text-sm">
					No project data available
				</div>
			{/if}
		</div>
	</div>
</div>

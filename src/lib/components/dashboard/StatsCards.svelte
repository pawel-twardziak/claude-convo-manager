<script lang="ts">
	import type { DashboardStats } from '$lib/types/db';
	import { formatTokens, formatCost } from '$lib/utils';

	let { stats }: { stats: DashboardStats } = $props();

	const cards = $derived([
		{ title: 'Sessions', value: stats.totalSessions.toString() },
		{ title: 'Projects', value: stats.totalProjects.toString() },
		{ title: 'Messages', value: formatTokens(stats.totalMessages) },
		{
			title: 'Tokens Used',
			value: formatTokens(
				stats.totalInputTokens +
					stats.totalOutputTokens +
					stats.totalCacheCreationTokens +
					stats.totalCacheReadTokens
			)
		},
		{ title: 'Est. Cost', value: formatCost(stats.estimatedTotalCost) },
		{ title: 'Avg Messages/Session', value: Math.round(stats.avgMessageCount).toString() }
	]);
</script>

<div class="grid grid-cols-2 gap-4 md:grid-cols-3 lg:grid-cols-6">
	{#each cards as card (card.title)}
		<div class="bg-card text-card-foreground rounded-lg border shadow-sm">
			<div class="p-4 pb-2">
				<p class="text-muted-foreground text-xs font-medium">{card.title}</p>
			</div>
			<div class="px-4 pb-4">
				<div class="text-2xl font-bold">{card.value}</div>
			</div>
		</div>
	{/each}
</div>

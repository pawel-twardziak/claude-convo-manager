<script lang="ts">
	import { onMount } from 'svelte';
	import { getDashboardStats, getProjectBreakdown, getActivityData } from '$lib/api/analytics';
	import { getSessions } from '$lib/api/sessions';
	import StatsCards from '$lib/components/dashboard/StatsCards.svelte';
	import RecentSessions from '$lib/components/dashboard/RecentSessions.svelte';
	import ProjectChart from '$lib/components/dashboard/ProjectChart.svelte';
	import ActivityChart from '$lib/components/dashboard/ActivityChart.svelte';
	import type { DashboardStats, SessionWithProject } from '$lib/types/db';

	let stats: DashboardStats | null = $state(null);
	let statsLoading = $state(true);
	let projects: { name: string | null; sessions: number; tokens: number }[] = $state([]);
	let projectsLoading = $state(true);
	let activity: { date: string; count: number }[] = $state([]);
	let activityLoading = $state(true);
	let recentSessions: SessionWithProject[] = $state([]);
	let sessionsLoading = $state(true);

	onMount(() => {
		getDashboardStats()
			.then((s) => (stats = s))
			.catch((e) => console.error('Failed to load stats:', e))
			.finally(() => (statsLoading = false));

		getProjectBreakdown()
			.then((p) => (projects = p))
			.catch((e) => console.error('Failed to load projects:', e))
			.finally(() => (projectsLoading = false));

		getActivityData()
			.then((a) => (activity = a))
			.catch((e) => console.error('Failed to load activity:', e))
			.finally(() => (activityLoading = false));

		getSessions({ sortBy: 'modified_at', pageSize: 10 })
			.then((r) => (recentSessions = r.sessions))
			.catch((e) => console.error('Failed to load sessions:', e))
			.finally(() => (sessionsLoading = false));
	});
</script>

<div class="w-full space-y-6 p-6">
	<h2 class="text-xl font-semibold">Dashboard</h2>

	{#if statsLoading}
		<div class="grid grid-cols-2 gap-4 md:grid-cols-3 lg:grid-cols-6">
			{#each Array(6) as _, i (i)}
				<div class="bg-card h-20 animate-pulse rounded-lg border p-4 shadow-sm"></div>
			{/each}
		</div>
	{:else if stats}
		<StatsCards {stats} />
	{/if}

	<div class="grid grid-cols-1 gap-6 lg:grid-cols-2">
		{#if projectsLoading}
			<div class="bg-card h-[370px] animate-pulse rounded-lg border shadow-sm"></div>
		{:else}
			<ProjectChart data={projects} />
		{/if}

		{#if activityLoading}
			<div class="bg-card h-[370px] animate-pulse rounded-lg border shadow-sm"></div>
		{:else}
			<ActivityChart data={activity} />
		{/if}
	</div>

	{#if sessionsLoading}
		<div class="bg-card rounded-lg border shadow-sm">
			<div class="p-4 pb-2">
				<div class="bg-muted h-5 w-48 animate-pulse rounded"></div>
			</div>
			<div class="space-y-1 px-4 pb-4">
				{#each Array(5) as _, i (i)}
					<div class="bg-muted h-10 animate-pulse rounded-md"></div>
				{/each}
			</div>
		</div>
	{:else}
		<RecentSessions sessions={recentSessions} />
	{/if}
</div>

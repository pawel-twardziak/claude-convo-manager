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
	let projects: { name: string | null; sessions: number; tokens: number }[] = $state([]);
	let activity: { date: string; count: number }[] = $state([]);
	let recentSessions: SessionWithProject[] = $state([]);
	let loading = $state(true);

	onMount(async () => {
		try {
			const [s, p, a, r] = await Promise.all([
				getDashboardStats(),
				getProjectBreakdown(),
				getActivityData(),
				getSessions({ sortBy: 'modified_at', pageSize: 10 })
			]);
			stats = s;
			projects = p;
			activity = a;
			recentSessions = r.sessions;
		} catch (e) {
			console.error('Failed to load dashboard:', e);
		} finally {
			loading = false;
		}
	});
</script>

<div class="w-full space-y-6 p-6">
	<h2 class="text-xl font-semibold">Dashboard</h2>

	{#if loading}
		<div class="grid grid-cols-2 gap-4 md:grid-cols-3 lg:grid-cols-6">
			{#each Array(6) as _, i (i)}
				<div class="bg-card h-20 animate-pulse rounded-lg border p-4 shadow-sm"></div>
			{/each}
		</div>
	{:else if stats}
		<StatsCards {stats} />
		<div class="grid grid-cols-1 gap-6 lg:grid-cols-2">
			<ProjectChart data={projects} />
			<ActivityChart data={activity} />
		</div>
		<RecentSessions sessions={recentSessions} />
	{/if}
</div>

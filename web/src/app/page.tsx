import { getDashboardStats } from "@/lib/actions/analytics";
import { getProjectBreakdown, getActivityData } from "@/lib/actions/analytics";
import { getSessions } from "@/lib/actions/sessions";
import { StatsCards } from "@/components/dashboard/stats-cards";
import { RecentSessions } from "@/components/dashboard/recent-sessions";
import { ProjectChart } from "@/components/dashboard/project-chart";
import { ActivityChart } from "@/components/dashboard/activity-chart";

export default async function DashboardPage() {
  const [stats, projects, activity, recent] = await Promise.all([
    getDashboardStats(),
    getProjectBreakdown(),
    getActivityData(),
    getSessions({ sortBy: "modified_at", pageSize: 10 }),
  ]);

  return (
    <div className="p-6 space-y-6 max-w-7xl">
      <h2 className="text-xl font-semibold">Dashboard</h2>
      <StatsCards stats={stats} />
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <ProjectChart data={projects} />
        <ActivityChart data={activity} />
      </div>
      <RecentSessions sessions={recent.sessions} />
    </div>
  );
}

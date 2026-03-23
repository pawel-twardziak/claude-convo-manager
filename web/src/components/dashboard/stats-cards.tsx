import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import type { DashboardStats } from "@/types/db";

function formatNumber(n: number): string {
  if (n >= 1_000_000) return `${(n / 1_000_000).toFixed(1)}M`;
  if (n >= 1_000) return `${(n / 1_000).toFixed(1)}K`;
  return n.toString();
}

export function StatsCards({ stats }: { stats: DashboardStats }) {
  const cards = [
    { title: "Sessions", value: stats.totalSessions.toString() },
    { title: "Projects", value: stats.totalProjects.toString() },
    { title: "Messages", value: formatNumber(stats.totalMessages) },
    {
      title: "Tokens Used",
      value: formatNumber(stats.totalInputTokens + stats.totalOutputTokens),
    },
    {
      title: "Est. Cost",
      value: `$${stats.estimatedTotalCost.toFixed(2)}`,
    },
    {
      title: "Avg Messages/Session",
      value: Math.round(stats.avgMessageCount).toString(),
    },
  ];

  return (
    <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4">
      {cards.map((card) => (
        <Card key={card.title}>
          <CardHeader className="pb-2">
            <CardTitle className="text-xs font-medium text-muted-foreground">
              {card.title}
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{card.value}</div>
          </CardContent>
        </Card>
      ))}
    </div>
  );
}

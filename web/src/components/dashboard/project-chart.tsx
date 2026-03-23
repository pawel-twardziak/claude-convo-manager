"use client";

import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  Tooltip,
  ResponsiveContainer,
} from "recharts";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

interface ProjectData {
  name: string;
  sessions: number;
  tokens: number;
}

export function ProjectChart({ data }: { data: ProjectData[] }) {
  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-base">Sessions by Project</CardTitle>
      </CardHeader>
      <CardContent>
        <div className="h-[300px]">
          <ResponsiveContainer width="100%" height="100%">
            <BarChart data={data} layout="vertical" margin={{ left: 80 }}>
              <XAxis type="number" fontSize={12} />
              <YAxis
                type="category"
                dataKey="name"
                fontSize={11}
                width={75}
                tickLine={false}
              />
              <Tooltip
                contentStyle={{
                  fontSize: 12,
                  borderRadius: 8,
                  border: "1px solid var(--border)",
                }}
              />
              <Bar
                dataKey="sessions"
                fill="var(--color-primary)"
                radius={[0, 4, 4, 0]}
              />
            </BarChart>
          </ResponsiveContainer>
        </div>
      </CardContent>
    </Card>
  );
}

"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { Button } from "@/components/ui/button";

export function SyncButton() {
  const [syncing, setSyncing] = useState(false);
  const router = useRouter();

  async function handleSync() {
    setSyncing(true);
    try {
      await fetch("/api/sync", { method: "POST" });
      router.refresh();
    } finally {
      setSyncing(false);
    }
  }

  return (
    <Button
      variant="ghost"
      size="sm"
      className="h-7 px-2 text-[11px] cursor-pointer"
      onClick={handleSync}
      disabled={syncing}
      title="Re-sync conversations from ~/.claude"
    >
      {syncing ? "Syncing..." : "Sync"}
    </Button>
  );
}

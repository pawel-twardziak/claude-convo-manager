"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { cn } from "@/lib/utils";
import { ThemeToggle } from "./theme-toggle";
import { SyncButton } from "./sync-button";

const navItems = [
  { href: "/", label: "Dashboard", icon: "◆" },
  { href: "/conversations", label: "Conversations", icon: "◇" },
  { href: "/search", label: "Search", icon: "⌕" },
];

export function Sidebar() {
  const pathname = usePathname();

  return (
    <aside className="w-56 shrink-0 border-r bg-card flex flex-col h-full">
      <div className="p-4 border-b">
        <h1 className="font-semibold text-sm tracking-tight">
          Claude Conversations
        </h1>
      </div>
      <nav className="flex-1 p-2 space-y-0.5">
        {navItems.map((item) => {
          const isActive =
            item.href === "/"
              ? pathname === "/"
              : pathname.startsWith(item.href);
          return (
            <Link
              key={item.href}
              href={item.href}
              className={cn(
                "flex items-center gap-2 px-3 py-2 rounded-md text-sm transition-colors",
                isActive
                  ? "bg-primary text-primary-foreground"
                  : "text-muted-foreground hover:bg-accent hover:text-accent-foreground"
              )}
            >
              <span className="text-base">{item.icon}</span>
              {item.label}
            </Link>
          );
        })}
      </nav>
      <div className="p-3 border-t text-xs text-muted-foreground flex items-center justify-between">
        <span>Claude Code Manager</span>
        <span className="inline-flex gap-1">
          <SyncButton />
          <ThemeToggle />
        </span>
      </div>
    </aside>
  );
}

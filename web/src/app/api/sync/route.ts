import { NextResponse } from "next/server";
import { fullSync } from "@/lib/sync/sync-engine";
import { getDb } from "@/lib/db/connection";

export async function POST() {
  try {
    getDb(); // ensure initialized
    await fullSync();

    const db = getDb();
    const sessions = db
      .prepare("SELECT COUNT(*) as count FROM sessions")
      .get() as { count: number };
    const messages = db
      .prepare("SELECT COUNT(*) as count FROM messages")
      .get() as { count: number };

    return NextResponse.json({
      ok: true,
      sessions: sessions.count,
      messages: messages.count,
    });
  } catch (err) {
    return NextResponse.json(
      { ok: false, error: String(err) },
      { status: 500 }
    );
  }
}

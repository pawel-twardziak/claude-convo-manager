import { notFound } from "next/navigation";
import { getSession } from "@/lib/actions/sessions";
import { getSessionMessages } from "@/lib/actions/messages";
import { SessionHeader } from "@/components/viewer/session-header";
import { MessageThread } from "@/components/viewer/message-thread";

interface Props {
  params: Promise<{ sessionId: string }>;
}

export default async function ConversationViewerPage({ params }: Props) {
  const { sessionId } = await params;
  const [session, result] = await Promise.all([
    getSession(sessionId),
    getSessionMessages({ sessionId, limit: 500 }),
  ]);

  if (!session) notFound();

  return (
    <div className="flex flex-col h-full">
      <SessionHeader session={session} />
      <MessageThread messages={result.messages} />
    </div>
  );
}

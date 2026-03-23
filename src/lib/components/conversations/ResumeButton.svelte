<script lang="ts">
	import { writeText } from '@tauri-apps/plugin-clipboard-manager';

	let { sessionId }: { sessionId: string } = $props();
	let copyState: 'idle' | 'copied-cmd' | 'copied-id' = $state('idle');

	async function copy(text: string, feedback: 'copied-cmd' | 'copied-id', e: MouseEvent) {
		e.preventDefault();
		e.stopPropagation();
		await writeText(text);
		copyState = feedback;
		setTimeout(() => (copyState = 'idle'), 2000);
	}
</script>

<span class="inline-flex gap-1">
	<button
		class="h-7 px-2 text-[11px] rounded-md text-muted-foreground hover:bg-accent hover:text-accent-foreground cursor-pointer"
		onclick={(e) => copy(`claude --resume ${sessionId}`, 'copied-cmd', e)}
		title="Copy: claude --resume <id>"
	>
		{copyState === 'copied-cmd' ? 'Copied!' : 'Resume'}
	</button>
	<button
		class="h-7 px-2 text-[11px] font-mono rounded-md text-muted-foreground hover:bg-accent hover:text-accent-foreground cursor-pointer"
		onclick={(e) => copy(sessionId, 'copied-id', e)}
		title="Copy session ID"
	>
		{copyState === 'copied-id' ? 'Copied!' : 'ID'}
	</button>
</span>

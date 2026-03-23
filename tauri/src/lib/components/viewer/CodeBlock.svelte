<script lang="ts">
	import { writeText } from '@tauri-apps/plugin-clipboard-manager';

	let { language, code }: { language: string; code: string } = $props();
	let copied = $state(false);

	async function handleCopy() {
		await writeText(code);
		copied = true;
		setTimeout(() => (copied = false), 2000);
	}
</script>

<div class="relative group my-2 rounded-md border bg-muted/50 overflow-hidden">
	<div class="flex items-center justify-between px-3 py-1.5 border-b bg-muted/80">
		<span class="text-[10px] text-muted-foreground font-mono">{language || 'text'}</span>
		<button
			onclick={handleCopy}
			class="text-[10px] text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
		>
			{copied ? 'Copied!' : 'Copy'}
		</button>
	</div>
	<pre class="p-3 overflow-x-auto text-[13px] leading-relaxed"><code>{code}</code></pre>
</div>

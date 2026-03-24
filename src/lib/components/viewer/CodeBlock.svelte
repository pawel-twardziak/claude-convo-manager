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

<div class="group bg-muted/50 relative my-2 overflow-hidden rounded-md border">
	<div class="bg-muted/80 flex items-center justify-between border-b px-3 py-1.5">
		<span class="text-muted-foreground font-mono text-[10px]">{language || 'text'}</span>
		<button
			onclick={handleCopy}
			class="text-muted-foreground hover:text-foreground cursor-pointer text-[10px] transition-colors"
		>
			{copied ? 'Copied!' : 'Copy'}
		</button>
	</div>
	<pre class="overflow-x-auto p-3 text-[13px] leading-relaxed"><code>{code}</code></pre>
</div>

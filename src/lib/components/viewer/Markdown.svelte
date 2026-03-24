<script lang="ts">
	import { marked } from 'marked';
	import hljs from 'highlight.js';

	let { content }: { content: string } = $props();

	// Configure marked with syntax highlighting
	marked.setOptions({
		gfm: true,
		breaks: false
	});

	const renderer = new marked.Renderer();

	// Custom code block rendering with syntax highlighting
	renderer.code = function ({ text, lang }: { text: string; lang?: string }) {
		const language = lang && hljs.getLanguage(lang) ? lang : 'plaintext';
		const highlighted = hljs.highlight(text, { language }).value;
		return `<div class="code-block-wrapper my-2 rounded-md border bg-muted/50 overflow-hidden">
			<div class="flex items-center justify-between px-3 py-1.5 border-b bg-muted/80">
				<span class="text-[10px] text-muted-foreground font-mono">${language}</span>
				<button onclick="navigator.clipboard.writeText(this.closest('.code-block-wrapper').querySelector('code').textContent)" class="text-[10px] text-muted-foreground hover:text-foreground transition-colors cursor-pointer copy-btn">Copy</button>
			</div>
			<pre class="p-3 overflow-x-auto text-[13px] leading-relaxed"><code class="hljs language-${language}">${highlighted}</code></pre>
		</div>`;
	};

	renderer.codespan = function ({ text }: { text: string }) {
		return `<code class="bg-muted px-1 py-0.5 rounded text-[13px]">${text}</code>`;
	};

	let html = $derived(marked.parse(content, { renderer }) as string);
</script>

<div class="prose prose-sm dark:prose-invert max-w-none break-words [&_pre]:my-0 [&_pre]:bg-transparent [&_pre]:p-0">
	<!-- eslint-disable-next-line svelte/no-at-html-tags -- rendered markdown from marked.parse -->
	{@html html}
</div>

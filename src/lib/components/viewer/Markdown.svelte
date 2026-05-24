<script lang="ts">
	import { marked } from 'marked';
	import hljs from 'highlight.js';
	import DOMPurify from 'dompurify';

	let { content }: { content: string } = $props();

	// Configure marked with syntax highlighting
	marked.setOptions({
		gfm: true,
		breaks: false
	});

	const renderer = new marked.Renderer();

	// Custom code block rendering with syntax highlighting. The copy action is handled via
	// event delegation on the wrapper (see handleClick) rather than an inline `onclick`, so
	// DOMPurify can strip all event-handler attributes from the sanitized output.
	renderer.code = function ({ text, lang }: { text: string; lang?: string }) {
		const language = lang && hljs.getLanguage(lang) ? lang : 'plaintext';
		const highlighted = hljs.highlight(text, { language }).value;
		return `<div class="code-block-wrapper my-2 rounded-md border bg-muted/50 overflow-hidden">
			<div class="flex items-center justify-between px-3 py-1.5 border-b bg-muted/80">
				<span class="text-[10px] text-muted-foreground font-mono">${language}</span>
				<button type="button" class="copy-btn text-[10px] text-muted-foreground hover:text-foreground transition-colors cursor-pointer">Copy</button>
			</div>
			<pre class="p-3 overflow-x-auto text-[13px] leading-relaxed"><code class="hljs language-${language}">${highlighted}</code></pre>
		</div>`;
	};

	renderer.codespan = function ({ text }: { text: string }) {
		return `<code class="bg-muted px-1 py-0.5 rounded text-[13px]">${text}</code>`;
	};

	// Sanitize the rendered HTML before injecting it — conversation content is untrusted.
	let html = $derived(DOMPurify.sanitize(marked.parse(content, { renderer }) as string));

	function handleClick(event: MouseEvent) {
		const btn = (event.target as HTMLElement).closest('.copy-btn');
		if (!btn) return;
		const code = btn.closest('.code-block-wrapper')?.querySelector('code');
		if (code?.textContent) {
			navigator.clipboard.writeText(code.textContent);
		}
	}
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	class="prose prose-sm dark:prose-invert prose-code:before:content-none prose-code:after:content-none max-w-none break-words [&_pre]:my-0 [&_pre]:bg-transparent [&_pre]:p-0"
	onclick={handleClick}
>
	<!-- eslint-disable-next-line svelte/no-at-html-tags -- sanitized markdown from marked.parse + DOMPurify -->
	{@html html}
</div>

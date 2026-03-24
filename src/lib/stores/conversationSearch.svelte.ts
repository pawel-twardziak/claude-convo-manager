let isOpen = $state(false);
let query = $state('');
let matches: HTMLElement[] = $state([]);
let currentIndex = $state(-1);

function escapeRegex(s: string): string {
	return s.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

export function openSearch() {
	isOpen = true;
}

export function closeSearch(container?: HTMLElement) {
	isOpen = false;
	query = '';
	if (container) clearHighlights(container);
}

export function getIsOpen(): boolean {
	return isOpen;
}

export function getQuery(): string {
	return query;
}

export function getMatchCount(): number {
	return matches.length;
}

export function getCurrentIndex(): number {
	return currentIndex;
}

export function getMatchLabel(): string {
	if (!query || query.length < 2) return '';
	if (matches.length === 0) return 'No matches';
	return `${currentIndex + 1} of ${matches.length}`;
}

export function performHighlight(container: HTMLElement, searchQuery: string) {
	clearHighlights(container);
	query = searchQuery;

	if (!searchQuery || searchQuery.length < 2) {
		matches = [];
		currentIndex = -1;
		return;
	}

	const pattern = new RegExp(escapeRegex(searchQuery), 'gi');
	const found: HTMLElement[] = [];

	const walker = document.createTreeWalker(container, NodeFilter.SHOW_TEXT, {
		acceptNode(node) {
			const parent = node.parentElement;
			if (!parent) return NodeFilter.FILTER_REJECT;
			const tag = parent.tagName.toLowerCase();
			if (tag === 'script' || tag === 'style' || tag === 'textarea') {
				return NodeFilter.FILTER_REJECT;
			}
			return NodeFilter.FILTER_ACCEPT;
		}
	});

	const textNodes: Text[] = [];
	let node: Node | null;
	while ((node = walker.nextNode())) {
		textNodes.push(node as Text);
	}

	for (const textNode of textNodes) {
		const text = textNode.textContent || '';
		const nodeMatches: { index: number; length: number }[] = [];
		let match: RegExpExecArray | null;

		pattern.lastIndex = 0;
		while ((match = pattern.exec(text)) !== null) {
			nodeMatches.push({ index: match.index, length: match[0].length });
		}

		if (nodeMatches.length === 0) continue;

		const parent = textNode.parentNode;
		if (!parent) continue;

		const frag = document.createDocumentFragment();
		let lastIdx = 0;

		for (const m of nodeMatches) {
			if (m.index > lastIdx) {
				frag.appendChild(document.createTextNode(text.slice(lastIdx, m.index)));
			}
			const mark = document.createElement('mark');
			mark.setAttribute('data-search-highlight', '');
			mark.textContent = text.slice(m.index, m.index + m.length);
			found.push(mark);
			frag.appendChild(mark);
			lastIdx = m.index + m.length;
		}

		if (lastIdx < text.length) {
			frag.appendChild(document.createTextNode(text.slice(lastIdx)));
		}

		parent.replaceChild(frag, textNode);
	}

	matches = found;
	currentIndex = found.length > 0 ? 0 : -1;
	scrollToCurrentMatch();
}

export function clearHighlights(container: HTMLElement) {
	const marks = container.querySelectorAll('mark[data-search-highlight]');
	for (const mark of marks) {
		const parent = mark.parentNode;
		if (!parent) continue;
		const text = document.createTextNode(mark.textContent || '');
		parent.replaceChild(text, mark);
	}
	container.normalize();
	matches = [];
	currentIndex = -1;
}

export function nextMatch() {
	if (matches.length === 0) return;
	currentIndex = (currentIndex + 1) % matches.length;
	scrollToCurrentMatch();
}

export function prevMatch() {
	if (matches.length === 0) return;
	currentIndex = (currentIndex - 1 + matches.length) % matches.length;
	scrollToCurrentMatch();
}

function scrollToCurrentMatch() {
	for (const m of matches) {
		m.classList.remove('search-current');
	}
	if (currentIndex >= 0 && currentIndex < matches.length) {
		matches[currentIndex].classList.add('search-current');
		matches[currentIndex].scrollIntoView({ behavior: 'smooth', block: 'center' });
	}
}
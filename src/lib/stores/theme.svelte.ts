export type ThemeMode = 'light' | 'dark' | 'system';

let mode = $state<ThemeMode>('system');
let dark = $state(false);

const mediaQuery = typeof window !== 'undefined' ? window.matchMedia('(prefers-color-scheme: dark)') : null;

function applyDark(isDark: boolean) {
	dark = isDark;
	document.documentElement.classList.toggle('dark', isDark);
}

function onSystemChange(e: MediaQueryListEvent) {
	if (mode === 'system') applyDark(e.matches);
}

export function initTheme() {
	if (typeof window === 'undefined') return;
	const stored = localStorage.getItem('theme') as ThemeMode | null;
	mode = stored ?? 'system';

	if (mode === 'system') {
		applyDark(mediaQuery!.matches);
	} else {
		applyDark(mode === 'dark');
	}

	mediaQuery!.addEventListener('change', onSystemChange);
}

export function setTheme(next: ThemeMode) {
	mode = next;
	if (next === 'system') {
		localStorage.removeItem('theme');
		applyDark(mediaQuery!.matches);
	} else {
		localStorage.setItem('theme', next);
		applyDark(next === 'dark');
	}
}

export function cycleTheme() {
	const order: ThemeMode[] = ['system', 'light', 'dark'];
	setTheme(order[(order.indexOf(mode) + 1) % 3]);
}

export function getMode(): ThemeMode {
	return mode;
}

export function isDark(): boolean {
	return dark;
}

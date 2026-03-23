let dark = $state(false);

export function initTheme() {
	if (typeof window === 'undefined') return;
	const stored = localStorage.getItem('theme');
	if (stored === 'dark' || (!stored && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
		dark = true;
		document.documentElement.classList.add('dark');
	}
}

export function toggleTheme() {
	dark = !dark;
	if (dark) {
		document.documentElement.classList.add('dark');
		localStorage.setItem('theme', 'dark');
	} else {
		document.documentElement.classList.remove('dark');
		localStorage.setItem('theme', 'light');
	}
}

export function isDark(): boolean {
	return dark;
}

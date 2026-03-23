import { clsx, type ClassValue } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

export function formatDate(dateStr: string | null): string {
	if (!dateStr) return '';
	const d = new Date(dateStr);
	return d.toLocaleDateString('en-US', {
		month: 'short',
		day: 'numeric',
		year: 'numeric',
		hour: '2-digit',
		minute: '2-digit'
	});
}

export function formatTokens(n: number): string {
	if (n >= 1_000_000) return `${(n / 1_000_000).toFixed(1)}M`;
	if (n >= 1_000) return `${(n / 1_000).toFixed(1)}K`;
	return String(n);
}

export function formatNumber(n: number): string {
	return n.toLocaleString();
}

export function formatCost(n: number): string {
	return `$${n.toFixed(2)}`;
}

export function timeAgo(dateStr: string | null): string {
	if (!dateStr) return '';
	const now = Date.now();
	const then = new Date(dateStr).getTime();
	const diff = now - then;

	const seconds = Math.floor(diff / 1000);
	if (seconds < 60) return 'just now';

	const minutes = Math.floor(seconds / 60);
	if (minutes < 60) return `${minutes}m ago`;

	const hours = Math.floor(minutes / 60);
	if (hours < 24) return `${hours}h ago`;

	const days = Math.floor(hours / 24);
	if (days < 30) return `${days}d ago`;

	const months = Math.floor(days / 30);
	return `${months}mo ago`;
}

<script lang="ts">
	interface ActivityData {
		date: string;
		count: number;
	}

	let { data }: { data: ActivityData[] } = $props();

	let maxCount = $derived(Math.max(...data.map((d) => d.count), 1));

	function formatDate(dateStr: string): string {
		const d = new Date(dateStr);
		return `${d.getMonth() + 1}/${d.getDate()}`;
	}

	// Build SVG path for area chart
	let svgPath = $derived.by(() => {
		if (data.length === 0) return { line: '', area: '' };
		const w = 100;
		const h = 100;
		const padding = 2;
		const step = (w - padding * 2) / Math.max(data.length - 1, 1);

		const points = data.map((d, i) => ({
			x: padding + i * step,
			y: h - padding - (d.count / maxCount) * (h - padding * 2)
		}));

		const line = points.map((p, i) => `${i === 0 ? 'M' : 'L'} ${p.x} ${p.y}`).join(' ');
		const area = `${line} L ${points[points.length - 1].x} ${h - padding} L ${points[0].x} ${h - padding} Z`;
		return { line, area };
	});
</script>

<div class="bg-card text-card-foreground rounded-lg border shadow-sm">
	<div class="p-4 pb-2">
		<h3 class="text-base font-semibold">Activity (Last 90 Days)</h3>
	</div>
	<div class="px-4 pb-4">
		<div class="h-[300px]">
			{#if data.length === 0}
				<div class="text-muted-foreground flex h-full items-center justify-center text-sm">
					No activity data available
				</div>
			{:else}
				<svg viewBox="0 0 100 100" preserveAspectRatio="none" class="h-full w-full">
					<path d={svgPath.area} fill="var(--color-primary)" fill-opacity="0.1" />
					<path d={svgPath.line} fill="none" stroke="var(--color-primary)" stroke-width="0.5" />
				</svg>
				<div class="text-muted-foreground mt-1 flex justify-between text-[10px]">
					{#if data.length > 0}
						<span>{formatDate(data[0].date)}</span>
						<span>{formatDate(data[data.length - 1].date)}</span>
					{/if}
				</div>
			{/if}
		</div>
	</div>
</div>

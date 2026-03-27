<script lang="ts">
	import { cycleTheme, getMode, isDark, initTheme } from '$lib/stores/theme.svelte';
	import { onMount } from 'svelte';

	onMount(() => {
		initTheme();
	});

	const icon = $derived(getMode() === 'system' ? '◑' : isDark() ? '☀' : '☾');
	const current = $derived(getMode() === 'system' ? 'System' : isDark() ? 'Dark' : 'Light');
	const next = $derived(getMode() === 'system' ? 'Light' : isDark() ? 'System' : 'Dark');
</script>

<button
	onclick={cycleTheme}
	title="{current} (switch to {next})"
	class="text-muted-foreground hover:bg-accent hover:text-accent-foreground flex h-7 w-7 cursor-pointer items-center justify-center rounded-md p-0 text-xs"
>
	{icon}
</button>

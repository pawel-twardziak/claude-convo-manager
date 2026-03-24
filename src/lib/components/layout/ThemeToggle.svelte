<script lang="ts">
	import { toggleTheme, isDark, initTheme } from '$lib/stores/theme.svelte';
	import { onMount } from 'svelte';

	onMount(() => {
		initTheme();
	});
</script>

<button
	onclick={toggleTheme}
	title={isDark() ? 'Light mode' : 'Dark mode'}
	class="text-muted-foreground hover:bg-accent hover:text-accent-foreground flex h-7 w-7 cursor-pointer items-center justify-center rounded-md p-0 text-xs"
>
	{isDark() ? '☀' : '☾'}
</button>

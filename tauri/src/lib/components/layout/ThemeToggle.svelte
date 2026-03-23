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
	class="h-7 w-7 p-0 text-xs rounded-md text-muted-foreground hover:bg-accent hover:text-accent-foreground cursor-pointer flex items-center justify-center"
>
	{isDark() ? '☀' : '☾'}
</button>

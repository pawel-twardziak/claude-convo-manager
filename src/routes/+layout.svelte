<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import Sidebar from '$lib/components/layout/Sidebar.svelte';
	import { loadDetectedApps } from '$lib/stores/ide.svelte';

	let { children } = $props();

	onMount(() => {
		loadDetectedApps();
	});
</script>

<Sidebar />
<main class="flex-1 overflow-auto">
	{@render children()}
</main>

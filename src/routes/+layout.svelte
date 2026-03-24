<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import Sidebar from '$lib/components/layout/Sidebar.svelte';
	import { loadDetectedApps } from '$lib/stores/ide.svelte';
	import { startSync } from '$lib/stores/sync.svelte';

	let { children } = $props();

	onMount(() => {
		loadDetectedApps();
		startSync();
	});
</script>

<Sidebar />
<main class="flex-1 overflow-auto">
	{@render children()}
</main>

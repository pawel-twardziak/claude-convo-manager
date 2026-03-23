<script lang="ts">
	import '../app.css';
	import Sidebar from '$lib/components/layout/Sidebar.svelte';

	let { children } = $props();
</script>

<Sidebar />
<main class="flex-1 overflow-auto">
	{@render children()}
</main>

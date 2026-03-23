<script lang="ts">
	import { goto } from '$app/navigation';

	let { defaultValue = '' }: { defaultValue?: string } = $props();
	let query = $state(defaultValue);

	function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		if (query.trim()) {
			goto(`/search?q=${encodeURIComponent(query.trim())}`);
		}
	}
</script>

<form onsubmit={handleSubmit} class="flex gap-2">
	<input
		bind:value={query}
		placeholder="Search across all conversations..."
		class="flex-1 h-9 rounded-md border border-input bg-background px-3 text-sm"
	/>
	<button
		type="submit"
		class="h-9 px-4 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90"
	>
		Search
	</button>
</form>

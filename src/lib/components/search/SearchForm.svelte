<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';

	let { defaultValue = '' }: { defaultValue?: string } = $props();
	let query = $state(defaultValue);

	function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		if (query.trim()) {
			goto(resolve(`/search?q=${encodeURIComponent(query.trim())}`));
		}
	}
</script>

<form onsubmit={handleSubmit} class="flex gap-2">
	<input
		bind:value={query}
		placeholder="Search across all conversations..."
		class="border-input bg-background h-9 flex-1 rounded-md border px-3 text-sm"
	/>
	<button
		type="submit"
		class="bg-primary text-primary-foreground hover:bg-primary/90 h-9 rounded-md px-4 text-sm font-medium"
	>
		Search
	</button>
</form>

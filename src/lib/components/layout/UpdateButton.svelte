<script lang="ts">
	import {
		getUpdateStatus,
		getUpdateVersion,
		getDownloadProgress,
		checkForUpdate,
		downloadAndInstall,
		restartApp
	} from '$lib/stores/updater.svelte';

	function handleClick() {
		const s = getUpdateStatus();
		if (s === 'idle' || s === 'up-to-date' || s === 'error') {
			checkForUpdate();
		} else if (s === 'available') {
			downloadAndInstall();
		} else if (s === 'ready') {
			restartApp();
		}
	}

	function getLabel(): string {
		const s = getUpdateStatus();
		switch (s) {
			case 'idle':
				return '⇡';
			case 'checking':
				return '…';
			case 'available':
				return '⬆';
			case 'downloading':
				return `${getDownloadProgress()}%`;
			case 'ready':
				return '⟳';
			case 'up-to-date':
				return '✓';
			case 'error':
				return '⚠';
			default:
				return '⇡';
		}
	}

	function getTitle(): string {
		const s = getUpdateStatus();
		switch (s) {
			case 'idle':
				return 'Check for updates';
			case 'checking':
				return 'Checking for updates...';
			case 'available':
				return `Update ${getUpdateVersion()} available — click to install`;
			case 'downloading':
				return `Downloading update... ${getDownloadProgress()}%`;
			case 'ready':
				return 'Update installed — click to restart';
			case 'up-to-date':
				return "You're up to date";
			case 'error':
				return 'Update check failed — click to retry';
			default:
				return 'Check for updates';
		}
	}
</script>

<button
	onclick={handleClick}
	disabled={getUpdateStatus() === 'checking' || getUpdateStatus() === 'downloading'}
	title={getTitle()}
	class="text-muted-foreground hover:bg-accent hover:text-accent-foreground flex h-7 w-7 cursor-pointer items-center justify-center rounded-md p-0 text-xs disabled:opacity-50"
	class:text-green-500={getUpdateStatus() === 'available' || getUpdateStatus() === 'ready'}
	class:text-red-400={getUpdateStatus() === 'error'}
>
	{getLabel()}
</button>

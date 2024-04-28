	<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';

	/**
		 * @type {string | any[]}
		 */
	let drives = [];

	async function showDrives() {
		const drivesJson = await invoke('show_drives');
		drives = JSON.parse(drivesJson);
	}

	onMount(async () => {
		await showDrives();
	});
	</script>

	<div>
	<button on:click="{showDrives}">Show drives</button>
	{#if drives.length > 0}
		{#each drives as drive}
		<div class="drive-container">
			<p class="drive-info"><span>Name:</span> {drive.name}</p>
			<p class="drive-info"><span>Disk Type:</span> {drive["disk type"]}</p>
			<p class="drive-info"><span>Mount Point:</span> {drive["mount point"]}</p>
			<p class="drive-info"><span>File System Type:</span> {drive["fs type"]}</p>
			<p class="drive-info"><span>External:</span> {drive.external ? 'Yes' : 'No'}</p>
			<p class="drive-info"><span>Size:</span> {drive.size}</p>
			<p class="drive-info"><span>Available:</span> {drive.avaliable}</p>
		</div>
		{/each}
	{:else}
		<p>No drives found.</p>
	{/if}
	</div>

	<style>
	.drive-container {
		border: 1px solid #ccc;
		border-radius: 5px;
		padding: 10px;
		margin-bottom: 10px;
	}

	.drive-info {
		font-size: 16px;
		margin-bottom: 5px;
	}

	.drive-info span {
		font-weight: bold;
	}
	</style>
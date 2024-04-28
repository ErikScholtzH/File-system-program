<script>
	import Counter from './Counter.svelte';
	import Drives from '../lib/Drives.svelte'
</script>

<svelte:head>
	<title>Home</title>
	<meta name="file-system-program" content="File system program" />
</svelte:head>

<section>

	<Drives />

	<Counter />
</section>

<style>
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 0.6;
	}

</style>

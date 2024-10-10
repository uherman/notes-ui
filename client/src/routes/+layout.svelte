<script lang="ts">
	import DarkModeToggle from '$lib/components/darkmode-toggle.svelte';
	import { onMount } from 'svelte';
	import '../app.css';
	import '../markdown.css';
	import 'highlight.js/styles/github.css';
	import { ModeWatcher } from 'mode-watcher';
	import { tryReconnect } from '$lib/utils/noteUtils';
	import { toggleViewMode, viewMode } from '$lib/stores';
	import { Toaster } from 'svelte-sonner';

	onMount(() => {
		tryReconnect();
	});
</script>

<ModeWatcher />
<Toaster richColors position="top-center" />

<main class="main">
	<nav class="navbar">
		<h1>Notes.md</h1>
		<div class="flex flex-row" style="gap: 30px; align-items:center;">
			<button
				class="btn-primary"
				style="width:90px;text-transform: capitalize;"
				on:click={toggleViewMode}>{$viewMode}</button
			>
			<DarkModeToggle />
		</div>
	</nav>
	<slot />
</main>

<style>
	.navbar {
		width: 100%;
		padding: 20px;
		display: flex;
		justify-content: space-between;
		background-color: var(--color-muted);
	}

	.main {
		height: 100vh;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
	}
</style>

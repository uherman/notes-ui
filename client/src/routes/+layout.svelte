<script lang="ts">
	import DarkModeToggle from '$lib/components/darkmode-toggle.svelte';
	import '../app.css';
	import '../markdown.css';
	import 'highlight.js/styles/github.css';
	import { ModeWatcher } from 'mode-watcher';
	import { tryReconnect } from '$lib/utils/noteUtils';
	import { canReconnect, connected, toggleViewMode, viewMode } from '$lib/stores';
	import { Toaster } from 'svelte-sonner';
</script>

<ModeWatcher />
<Toaster richColors position="top-center" duration={1200} />

<main class="main">
	<nav class="navbar">
		<h1>Notes.md</h1>
		<div class="flex flex-row" style="gap: 30px; align-items:center;">
			{#if $connected}
				<div class="indicator-btn" style="cursor: initial;">
					<span class="indicator bg-success" />
				</div>
			{:else if $canReconnect}
				<button class="indicator-btn" on:click={() => tryReconnect('Reconnecting...')}>
					<span class="indicator bg-danger" />
				</button>
			{:else}
				<div class="indicator-btn" style="cursor: initial;">
					<span class="indicator bg-overlay" />
				</div>
			{/if}
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
	.indicator-btn {
		background: none;
		border: none;
		cursor: pointer;
		padding: 0.5rem;
		display: flex;
		flex-direction: row;
		align-items: center;
	}
	.indicator {
		display: inline-block;
		width: 10px;
		height: 10px;
		border-radius: 50%;
	}
	.indicator.bg-overlay {
		animation: pulse 1s infinite;
	}
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

	@keyframes pulse {
		50% {
			background-color: var(--color-attention);
		}
	}
</style>

<script lang="ts">
	import DarkModeToggle from '$lib/components/darkmode-toggle.svelte';
	import '../app.css';
	import '../markdown.css';
	import 'highlight.js/styles/github.css';
	import { ModeWatcher } from 'mode-watcher';
	import { tryReconnect } from '$lib/utils/websocketUtils';
	import { canReconnect, connected, toggleViewMode, viewMode } from '$lib/stores';
	import { Toaster } from 'svelte-sonner';

	let hasLoaded = false;

	$: !hasLoaded && $canReconnect && (hasLoaded = true);
</script>

<ModeWatcher />
<Toaster richColors position="top-center" duration={1200} />

<main class="main">
	<nav class="navbar">
		<h1>Notes.md</h1>
		<div class="flex flex-row nav-right">
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
	{#if hasLoaded}
		<span class="h-full w-full fade overflow-hidden">
			<slot />
		</span>
	{:else}
		<div class="flex flex-col items-center justify-center h-full w-full">
			<div
				style="display:flex; justify-content:center; align-items:center; margin-top: 30px; padding: 3rem; flex-direction: column; gap: 40px;"
			>
				<p>You are stuck in the matrix</p>
				<a
					class="btn-primary w-full"
					style="text-align: center; text-decoration:none;"
					href="/login">Login</a
				>
			</div>
		</div>
	{/if}
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
		align-items: center;
		background-color: var(--color-muted);
	}
	.nav-right {
		gap: 30px;
		align-items: center;
	}

	@media (max-width: 768px) {
		.navbar {
			padding-left: 70px;
		}
		.nav-right {
			gap: 10px;
		}
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

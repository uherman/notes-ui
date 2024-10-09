<script lang="ts">
	import DarkModeToggle from '$lib/components/darkmode-toggle.svelte';
	import { onMount } from 'svelte';
	import '../app.css';
	import '../markdown.css';
	import 'highlight.js/styles/github.css';
	import { ModeWatcher } from 'mode-watcher';
	import { connect, tryReconnect } from '$lib/utils/noteUtils';
	import { signedIn, toggleViewMode, viewMode } from '$lib/stores';
	import { toast, Toaster } from 'svelte-sonner';

	let loaded = false;
	let value = '';

	onMount(() => {
		tryReconnect();
		const timeout = setTimeout(() => {
			loaded = true;
			clearTimeout(timeout);
		}, 300);
	});

	const authenticate = () => {
		connect(value);

		const timeout = setTimeout(() => {
			if (!$signedIn) {
				toast.error('Unauthorized');
			}
			clearTimeout(timeout);
		}, 300);
	};
</script>

<ModeWatcher />
<Toaster richColors position="top-center" />

{#if loaded}
	{#if !$signedIn}
		<main class="main">
			<form on:submit|preventDefault={authenticate} class="login-form">
				<input type="password" bind:value placeholder="Enter token" />
			</form>
		</main>
	{:else}
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
	{/if}
{/if}

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
	.login-form {
		display: flex;
		flex-direction: row;
		align-items: start;
		justify-content: center;
		gap: 10px;
		width: 100%;
		height: 100%;
		margin-top: 200px;
	}

	.login-form input {
		width: 300px;
		padding: 10px;
		border: 1px solid var(--color-border);
		background-color: var(--color-muted);
		border-radius: 5px;
		outline: none;
	}
</style>

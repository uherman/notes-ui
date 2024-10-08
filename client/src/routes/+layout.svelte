<script lang="ts">
	import DarkModeToggle from '$lib/components/darkmode-toggle.svelte';
	import '../app.css';
	import '../markdown.css';
	import 'highlight.js/styles/github.css';

	import { ModeWatcher } from 'mode-watcher';
</script>

<ModeWatcher />

<main class="main">
	<nav class="navbar">
		<h1>Notes.md</h1>
		<DarkModeToggle />
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

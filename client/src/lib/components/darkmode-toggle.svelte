<script lang="ts">
	import { toggleMode, mode } from 'mode-watcher';
	import Sun from 'lucide-svelte/icons/sun';
	import Moon from 'lucide-svelte/icons/moon';
</script>

<button class="toggle" on:click={toggleMode}>
	{#if $mode === 'dark'}
		<Moon class="text-attention" />
	{:else}
		<Sun class="text-attention" />
	{/if}
</button>

<style>
	.toggle {
		background: none;
		border: none;
		cursor: pointer;
		font-size: 1rem;
	}
</style>

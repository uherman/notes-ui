<script lang="ts">
	import { Copy, CopyCheck } from 'lucide-svelte';

	let element: HTMLPreElement;
	let copied = false;

	const getTextContent = (): string => {
		if (!element) return '';
		let textContent: string = '';

		element.children[0].childNodes.forEach((node: Node) => {
			textContent += node.textContent;
		});

		return textContent.trim();
	};

	const copy = () => {
		navigator.clipboard.writeText(getTextContent());
		copied = true;
		const timeout = setTimeout(() => {
			copied = false;
			clearTimeout(timeout);
		}, 800);
	};
</script>

<pre class="flex relative" bind:this={element}>
	<slot />
    <button class={`copy-btn ${copied ? 'copied' : ''}`} on:click={copy}>
        {#if copied}
			<CopyCheck size="20" />
		{:else}
			<Copy size="20" />
		{/if}
    </button>
</pre>

<style>
	pre:hover {
		padding-right: 30px;
	}

	pre:hover .copy-btn {
		opacity: 1;
	}
	.copy-btn {
		display: flex;
		position: absolute;
		right: 10px;
		top: 12px;
		background: none;
		border: none;
		cursor: pointer;
		margin-left: 10px;
		color: var(--color-text);
		padding: 0.3rem;
		border-radius: 5px;
		transition: 0.2s;
		opacity: 0;
		background-color: var(--color-muted);
	}

	.copy-btn:hover {
		color: var(--color-primary);
		background-color: var(--color-muted);
	}
	.copy-btn:active {
		transform: scale(0.8);
	}
	.copy-btn.copied {
		color: var(--color-success);
	}
</style>

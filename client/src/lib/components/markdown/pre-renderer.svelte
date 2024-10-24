<script lang="ts">
	import { Copy } from 'lucide-svelte';
	import { toast } from 'svelte-sonner';

	let element: HTMLPreElement;

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
		toast.success('Copied to clipboard');
	};
</script>

<pre class="flex relative" bind:this={element}>
	<slot />
    <button class="copy-btn" on:click={copy}>
        <Copy size="20" />
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
</style>

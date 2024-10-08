<script lang="ts">
	import { onMount } from 'svelte';

	export let message = 'Are you sure?';
	export let onConfirm: () => void;
	export let onCancel: () => void;

	let modalElement: HTMLButtonElement;

	const handleKeyDown = (event: KeyboardEvent) => {
		if (event.key === 'Escape') {
			onCancel();
		}

		if (event.key === 'Enter') {
			onConfirm();
		}
	};

	onMount(() => {
		modalElement.focus();
	});
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="modal-backdrop" on:click={onCancel} on:keydown={handleKeyDown}>
	<button class="modal" bind:this={modalElement} on:click|stopPropagation>
		<h3 class="text-default bold">{message}</h3>
		<div class="modal-actions">
			<button class="confirm-btn" on:click={onConfirm}>Yes</button>
			<button class="cancel-btn" on:click={onCancel}>No</button>
		</div>
	</button>
</div>

<style>
	/* Modal backdrop */
	.modal-backdrop {
		position: fixed;
		top: 0;
		left: 0;
		width: 100vw;
		height: 100vh;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		justify-content: center;
		align-items: flex-start; /* Align modal to the top */
		padding-top: 200px; /* Add padding from the top */
	}

	/* Modal box */
	.modal {
		background-color: var(--color-bg);
		padding: 30px;
		padding-bottom: 20px;
		border-radius: 10px;
		box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
		max-width: 400px;
		width: 100%;
		text-align: center;
	}

	/* Modal actions */
	.modal-actions {
		margin-top: 20px;
		display: flex;
		justify-content: center;
		gap: 10%;
	}

	/* Confirm and Cancel buttons */
	.confirm-btn {
		background-color: var(--color-danger);
		color: var(--color-bg);
		border: none;
		padding: 10px 20px;
		border-radius: 5px;
		cursor: pointer;
	}

	.cancel-btn {
		background-color: var(--color-muted);
		color: var(--color-text);
		border: none;
		padding: 10px 20px;
		border-radius: 5px;
		cursor: pointer;
	}

	.confirm-btn:hover,
	.cancel-btn:hover {
		opacity: 0.9;
	}
</style>

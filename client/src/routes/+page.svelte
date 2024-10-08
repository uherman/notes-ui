<script lang="ts">
	import Editor from '$lib/components/markdown/editor.svelte';
	import type { Note } from '$lib/types';
	import { deleteNote, getNotes, saveNote } from '$lib/utils/noteUtils';
	import { nanoid } from 'nanoid';
	import { onMount } from 'svelte';
	import Modal from '$lib/components/modal.svelte';

	let note: Note | undefined;
	let notes: Note[] = [];

	let showModal = false;
	let noteToDelete: Note | undefined;

	onMount(async () => {
		notes = await getNotes();
		if (notes.length > 0) {
			note = notes[0];
		}
	});

	const selectNote = (n: Note) => {
		note = n;
	};

	const createNewNote = () => {
		note = {
			id: nanoid(),
			content: '# New Note\n\nStart writing your note here...',
			updated: new Date()
		};
		saveNote(note);
		notes = [note, ...notes];
	};

	const save = async (note: Note) => {
		await saveNote(note);
		notes = await getNotes();
	};

	const getTitle = (content: string) => {
		const title = content.split('\n')[0];
		return title.slice(2);
	};

	const confirmDelete = (note: Note) => {
		noteToDelete = note;
		showModal = true;
	};

	const closeModal = () => {
		showModal = false;
		noteToDelete = undefined;
	};

	const removeNote = async () => {
		if (!noteToDelete) {
			closeModal();
			return;
		}

		notes = notes.filter((n) => n.id !== noteToDelete?.id);
		await deleteNote(noteToDelete.id);
		closeModal();
	};
</script>

<div class="h-full w-full relative">
	<div class="sidebar">
		<div class="new-note-btn-container">
			<button class="new-note-btn" on:click={createNewNote}>New Note</button>
		</div>
		<div class="notes-container">
			<ul>
				{#each notes as note}
					<li class="note-item">
						<button class="note" on:click={() => selectNote(note)}>
							<p class="note-title">
								{getTitle(note.content)}
							</p>
							<button class="delete-btn" on:click|stopPropagation={() => confirmDelete(note)}>
								&times;
							</button>
						</button>
					</li>
				{/each}
			</ul>
		</div>
	</div>

	{#if note}
		<div class="editor h-full">
			<Editor {note} {save} />
		</div>
	{/if}

	{#if showModal}
		<Modal
			message="Are you sure you want to delete this note?"
			onConfirm={removeNote}
			onCancel={closeModal}
		/>
	{/if}
</div>

<style>
	/* Editor */
	.editor {
		margin-left: 250px;
	}
	.new-note-btn-container {
		margin-top: 10px;
		padding-left: 20px;
		padding-right: 20px;
	}

	/* New Note Button */
	.new-note-btn {
		display: block;
		width: 100%;
		padding: 10px;
		font-size: 1rem;
		font-weight: bold;
		color: var(--color-primary);
		background-color: transparent;
		border-color: var(--color-primary) !important;
		border: 1px solid;
		border-radius: 5px;
		cursor: pointer;
		transition:
			background-color 0.2s ease,
			color 0.2s ease;
	}

	.new-note-btn:hover {
		background-color: var(--color-primary);
		color: var(--color-bg);
	}

	/* Sidebar container */
	.sidebar {
		position: absolute;
		left: 0;
		top: 0;
		bottom: 0;
		width: 250px; /* Adjust the width as needed */
		background-color: var(--color-muted);
		overflow-y: auto;
		border-right: 1px solid var(--color-border);
	}

	/* Notes container inside the sidebar */
	.notes-container {
		padding: 20px;
		background-color: var(--color-muted);
		color: var(--color-text);
		width: 100%;
	}

	ul {
		list-style-type: none;
		padding: 0;
		margin: 0;
	}

	/* Note item containing both the note and delete button */
	.note-item {
		display: flex;
		align-items: center;
		margin-bottom: 10px;
		background-color: transparent;
	}

	/* Note button */
	.note {
		flex-grow: 1;
		background: none;
		border: none;
		cursor: pointer;
		padding: 10px;
		text-align: left;
		color: var(--color-primary);
		font-size: 1rem;
		border-radius: 5px;
		transition: background-color 0.2s ease;
		font-weight: bold;
		display: flex;
		flex-direction: row;
		justify-content: space-between;
		text-overflow: ellipsis;
	}

	.note-title {
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		max-width: 150px;
	}

	.note:hover {
		background-color: var(--color-border);
	}

	/* Delete button (the small X) */
	.delete-btn {
		background: none;
		border: none;
		cursor: pointer;
		color: var(--color-danger);
		font-size: 1.2rem;
		padding: 0 0.2rem;
		transition: color 0.2s ease;
	}

	.delete-btn:hover {
		color: var(--color-danger-hover);
	}
</style>

import type { Note } from '$lib/types';

export const saveNote = async (note: Note) => {
	localStorage.setItem(
		`note:${note.id}`,
		JSON.stringify({ ...note, updated: new Date().toISOString() })
	);
};

export const loadNote = async (id: string): Promise<Note> => {
	const noteString = localStorage.getItem(`note:${id}`);
	if (!noteString) {
		throw new Error('Note not found');
	}

	const note: Note = JSON.parse(noteString);
	return {
		id: note.id,
		content: note.content,
		updated: new Date(note.updated)
	};
};

export const deleteNote = async (id: string) => {
	localStorage.removeItem(`note:${id}`);
};

export const getNoteIds = async (): Promise<string[]> => {
	const noteIds = [];
	for (let i = 0; i < localStorage.length; i++) {
		const key = localStorage.key(i);
		if (key?.startsWith('note:')) {
			noteIds.push(key.slice(5));
		}
	}

	return noteIds;
};

export const getNotes = async (): Promise<Note[]> => {
	const noteIds = await getNoteIds();
	const notes = await Promise.all(noteIds.map((id) => loadNote(id)));
	return notes.sort((a, b) => b.updated.getTime() - a.updated.getTime());
};

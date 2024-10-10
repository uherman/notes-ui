import type { Note } from '$lib/types';

export const sortNotes = (notes: Note[]) =>
	notes.sort((a: Note, b: Note) => new Date(b.updated).getTime() - new Date(a.updated).getTime());

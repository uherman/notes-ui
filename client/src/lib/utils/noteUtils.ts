import { signedIn } from '$lib/stores';
import type { Note } from '$lib/types';
import { toast } from 'svelte-sonner';
import { writable } from 'svelte/store';
import { PUBLIC_API_URL } from '$env/static/public';

enum Command {
	Get = 'Get',
	Set = 'Set',
	Delete = 'Delete'
}

export const notes = writable<Note[]>([]);

let ws: WebSocket;

export const tryReconnect = async () => {
	const userInfo = await fetch(`/Account/Profile`).then((res) => res.json());
	connect(userInfo.name);
};

export const connect = (username: string) => {
	ws = new WebSocket(`${PUBLIC_API_URL}?username=${username}`);
	ws.onclose = (event) => {
		console.log('Websocket closed:', event.code, event);
		signedIn.set(false);
	};

	ws.onmessage = (event) => {
		try {
			const data = JSON.parse(event.data);
			if (!data.response) {
				notes.set(data);
				signedIn.set(true);
			} else {
				if (data.response === 200) {
					ws.send(JSON.stringify({ command: Command.Get }));
				}
			}
		} catch {
			console.error('Error parsing notes' + event.data);
		}
	};

	ws.onopen = () => {
		ws.send(JSON.stringify({ command: Command.Get }));
	};
};

export const saveNote = async (note: Note) => {
	localStorage.setItem(
		`note:${note.id}`,
		JSON.stringify({ ...note, updated: new Date().toISOString() })
	);

	try {
		ws.send(
			JSON.stringify({ command: Command.Set, note: { ...note, updated: new Date().toISOString() } })
		);
	} catch (e) {
		console.error('Failed to save note:', e);
		toast.error('Failed to save note');
	}
};

export const deleteNote = async (id: string) => {
	localStorage.removeItem(`note:${id}`);
	ws.send(JSON.stringify({ command: Command.Delete, note: { id } }));
};

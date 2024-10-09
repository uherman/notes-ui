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

const getSessionToken = () => {
	// HACK: Using this until a more proper auth flow is implemented
	// Have a look at session based auth in rust:
	// - https://www.lpalmieri.com/posts/session-based-authentication-in-rust/
	// - https://www.shuttle.rs/blog/2022/08/11/authentication-tutorial
	const token = sessionStorage.getItem('token');
	if (token) {
		return window.atob(token).substring(2, token.length);
	}
};

export const tryReconnect = () => {
	const token = getSessionToken();
	if (token) {
		connect(token);
	}
};

export const connect = (token: string) => {
	ws = new WebSocket(`${PUBLIC_API_URL}?token=${token}`);
	ws.onclose = (event) => {
		console.log('Websocket closed:', event.code, event.reason);
		signedIn.set(false);
	};

	ws.onmessage = (event) => {
		try {
			const data = JSON.parse(event.data);
			if (!data.response) {
				notes.set(data);
				signedIn.set(true);

				sessionStorage.setItem('token', window.btoa('y6' + token));
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

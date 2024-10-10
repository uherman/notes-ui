import { canReconnect, connected, notes } from '$lib/stores';
import type { Note } from '$lib/types';
import { toast } from 'svelte-sonner';
import { PUBLIC_API_URL } from '$env/static/public';
import { sortNotes } from './noteUtils';
import { goto } from '$app/navigation';

enum Command {
	Get = 'Get',
	Set = 'Set',
	Delete = 'Delete'
}

let ws: WebSocket;

export const tryReconnect = async (
	loading: string = 'Connection lost, reconnecting...',
	error: string = 'Failed to reconnect... Try reloading the page.'
) => {
	canReconnect.set(false);
	setTimeout(() => {
		toast.promise(reconnect, {
			loading,
			success: () => {
				canReconnect.set(true);
				return 'Connected!';
			},
			error: () => {
				canReconnect.set(true);
				return error;
			}
		});
	}, 100);
};

async function reconnect() {
	for (let i = 0; i < 10; i++) {
		try {
			const userInfo = await fetch(`/Account/Profile`).then((res) => res.json());
			await connect(userInfo.name);
			return;
		} catch {
			await new Promise((resolve) => setTimeout(resolve, 1000));
		}
	}

	return Promise.reject('Failed to reconnect');
}

connected.subscribe((value) => {
	if (!value) {
		if (!ws) {
			canReconnect.set(false);
			reconnect()
				.catch(() => toast.error('Failed to connect'))
				.finally(() => canReconnect.set(true));
		} else {
			tryReconnect().catch(() => {
				console.error('Failed to reconnect');
			});
		}
	}
});

const connect = (username: string) => {
	return new Promise<void>((resolve, reject) => {
		ws = new WebSocket(`${PUBLIC_API_URL}?username=${username}`);
		ws.onclose = (event) => {
			console.log('Websocket closed:', event.code, event);
			connected.set(false);
			if (event.code === 401) {
				goto('/login');
			}
			reject();
		};

		ws.onmessage = (event) => {
			try {
				const data = JSON.parse(event.data);
				if (!data.response) {
					notes.set(sortNotes(data));
					connected.set(true);
					resolve();
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
	});
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

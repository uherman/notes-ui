import { writable } from 'svelte/store';

export const signedIn = writable(false);

const viewModes: ('split' | 'editor' | 'preview')[] = ['split', 'editor', 'preview'];
let viewModeIndex = 0;

export const viewMode = writable(viewModes[viewModeIndex]);

export const toggleViewMode = () => {
	viewModeIndex = (viewModeIndex + 1) % viewModes.length;
	viewMode.set(viewModes[viewModeIndex]);
};

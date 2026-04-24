import { writable } from 'svelte/store';

export const editorContent = writable('Once upon a time...');
export const storyHistory = writable([]);
export const compilerErrors = writable([]);
export const theme = writable('dark');

import { writable } from 'svelte/store';

/** @type {import('svelte/store').Writable<string>} */
export const editorContent = writable('Once upon a time...');
/** @type {import('svelte/store').Writable<any[]>} */
export const storyHistory = writable([]);
/** @type {import('svelte/store').Writable<any[]>} */
export const compilerErrors = writable([]);

const initialTheme = (typeof window !== 'undefined' && localStorage.getItem('theme')) || 'dark';
/** @type {import('svelte/store').Writable<string>} */
export const theme = writable(initialTheme);

if (typeof window !== 'undefined') {
    theme.subscribe(value => {
        localStorage.setItem('theme', value);
    });
}

import { writable } from 'svelte/store';

/** @type {import('svelte/store').Writable<string>} */
export const editorContent = writable('Once upon a time...');
/** @type {import('svelte/store').Writable<any[]>} */
export const storyHistory = writable([]);
/** @type {import('svelte/store').Writable<any[]>} */
export const compilerErrors = writable([]);

// New stores
/** @type {import('svelte/store').Writable<string[]>} */
export const projectFiles = writable([]);
/** @type {import('svelte/store').Writable<string|null>} */
export const activeFilePath = writable(null);
/** @type {import('svelte/store').Writable<string|null>} */
export const mainInkPath = writable(null);
/** @type {import('svelte/store').Writable<boolean>} */
export const sidebarVisible = writable(true);

const initialTheme = (typeof window !== 'undefined' && localStorage.getItem('theme')) || 'dark';
/** @type {import('svelte/store').Writable<string>} */
export const theme = writable(initialTheme);

if (typeof window !== 'undefined') {
    theme.subscribe(value => {
        localStorage.setItem('theme', value);
    });
}

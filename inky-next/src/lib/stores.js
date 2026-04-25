import { writable } from 'svelte/store';

export const editorContent = writable('Once upon a time...');
export const storyHistory = writable([]);
export const compilerErrors = writable([]);

const initialTheme = (typeof window !== 'undefined' && localStorage.getItem('theme')) || 'dark';
export const theme = writable(initialTheme);

if (typeof window !== 'undefined') {
    theme.subscribe(value => {
        localStorage.setItem('theme', value);
    });
}

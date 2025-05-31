import { writable } from 'svelte/store';

export const codeFiles = writable<string[]>([]);
export const isCodeViewerOpen = writable<boolean>(false);

export function registerCodeFiles(files: string[]) {
  codeFiles.set(files);
}

export function clearCodeFiles() {
  codeFiles.set([]);
}

export function setCodeViewerOpen(open: boolean) {
  isCodeViewerOpen.set(open);
}
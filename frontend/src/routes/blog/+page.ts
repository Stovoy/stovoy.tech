import type { PageLoad } from './$types';
import type { BlogMeta } from '$lib/markdown';
import { extractMeta } from '$lib/markdown';

const markdownFiles = import.meta.glob('content/*.md', {
  eager: true,
  as: 'raw'
}) as Record<string, string>;

const blogs: BlogMeta[] = Object.entries(markdownFiles).map(([path, markdown]) =>
  extractMeta(markdown, path)
);

blogs.sort((a, b) => b.date.localeCompare(a.date));

export const prerender = true;

export const load: PageLoad = async () => {
  return { blogs };
};
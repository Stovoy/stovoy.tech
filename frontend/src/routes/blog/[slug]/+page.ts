import type { PageLoad } from './$types';
import { toHtml, extractMeta } from '$lib/markdown';

const markdownFiles = import.meta.glob('content/*.md', {
  eager: true,
  as: 'raw'
}) as Record<string, string>;

export const prerender = true;

export const load: PageLoad = async ({ params }) => {
  const slug = params.slug;
  const entry = Object.entries(markdownFiles).find(([path]) => path.endsWith(`${slug}.md`));

  if (!entry) {
    throw new Error('Not found');
  }

  const [path, md] = entry;
  const meta = extractMeta(md, path);
  const html = toHtml(md);

  return { html, meta, source: path };
};

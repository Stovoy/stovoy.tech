import type { PageLoad } from './$types';
import { error } from '@sveltejs/kit';

export const load: PageLoad = async ({ params, fetch }) => {
  const res = await fetch(`/blog/${params.slug}/index.html`);
  if (res.ok) {
    const html = await res.text();
    return { html };
  }
  throw error(404, 'Not found');
};

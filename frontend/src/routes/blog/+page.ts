import type { PageLoad } from './$types';
import { error } from '@sveltejs/kit';

export const load: PageLoad = async ({ fetch }) => {
  const res = await fetch('/blog/blogs.json');
  if (res.ok) {
    const blogs = (await res.json()) as {
      title: string;
      date: string;
      slug: string;
    }[];
    return { blogs };
  }
  throw error(500, 'Blogs not found');
};

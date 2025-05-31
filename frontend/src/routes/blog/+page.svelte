<script lang="ts">
  export let data: { blogs: { title: string; date: string; slug: string }[] };
</script>

<h1 class="text-2xl font-bold mb-4">Blog</h1>

<ul class="space-y-2">
  {#each data.blogs as blog}
    <li>
      <a href={"/blog/" + blog.slug} class="text-blue-600 hover:underline">
        {blog.title}
      </a>
      <span class="text-sm text-gray-500 ml-2">{blog.date}</span>
    </li>
  {/each}
</ul>

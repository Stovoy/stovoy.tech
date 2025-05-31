<script lang="ts">
  const examples = [
    'backend/src/lib.rs',
    'frontend/src/routes/+page.svelte',
    'frontend/src/lib/components/TypingText.svelte',
    'frontend/src/lib/components/SnakeGame.svelte'
  ];
</script>

<h1 class="text-2xl font-bold mb-4">Source code viewer</h1>

<p class="mb-4 opacity-80">Click any file below or append <code>#src=path/to/file.rs</code> to the URL, then click the View Code button.</p>

<ul class="list-disc list-inside space-y-1">
  {#each examples as path}
    <li>
      <a href={`#src=${encodeURIComponent(path)}`} class="text-blue-600 dark:text-blue-400 hover:underline">{path}</a>
    </li>
  {/each}
</ul>
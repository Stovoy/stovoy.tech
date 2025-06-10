<script lang="ts">
  import { onMount } from 'svelte';
  import { registerCodeFiles } from '$lib/stores/codeFiles';

  export let data: { html: string; source: string };

  onMount(() => {
    const files: string[] = [];
    if (data.source) {
      let clean = data.source;
      while (clean.startsWith('../')) {
        clean = clean.slice(3);
      }
      files.push(clean);
    }
    files.push('frontend/src/routes/blog/[slug]/+page.svelte');
    files.push('frontend/src/routes/blog/[slug]/+page.ts');
    registerCodeFiles(files);
  });
</script>

<div class="back-button-container">
  <a href="/?instant=1" class="back-button">
    <span class="back-arrow">←</span>
    <span>Back</span>
  </a>
</div>

<article class="prose max-w-none">
  {@html data.html}
</article>

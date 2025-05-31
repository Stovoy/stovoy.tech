<script lang="ts">
  import { onMount } from 'svelte';
  import { registerCodeFiles } from '$lib/stores/codeFiles';

  export let data: { html: string; source: string };

  onMount(() => {
    if (data.source) {
      registerCodeFiles([data.source]);
    }
  });
</script>

<article class="prose max-w-none">
  {@html data.html}
</article>

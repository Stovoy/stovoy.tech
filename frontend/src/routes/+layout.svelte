<script>
  import '../app.css';
  import CodeViewer from '$lib/components/CodeViewer.svelte';
  import { afterNavigate } from '$app/navigation';
  import { clearCodeFiles, isCodeViewerOpen } from '$lib/stores/codeFiles';
  import { get } from 'svelte/store';

  afterNavigate(() => {
    // Only clear code files if the viewer is not open
    if (!get(isCodeViewerOpen)) {
      clearCodeFiles();
    }
  });
</script>

<svelte:head>
  <meta name="viewport" content="width=device-width, initial-scale=1" />
</svelte:head>

<main>
  <slot />
</main>

<CodeViewer />

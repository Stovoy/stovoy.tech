<script>
  import '../app.css';
  import CodeViewer from '$lib/components/CodeViewer.svelte';
  import { afterNavigate } from '$app/navigation';
  import { clearCodeFiles, isCodeViewerOpen } from '$lib/stores/codeFiles';
  import { get } from 'svelte/store';

  afterNavigate(() => {
    if (!get(isCodeViewerOpen)) {
      clearCodeFiles();
    }
    if (window.hljs) {
      window.hljs.highlightAll();
    }
    if (window.mermaid && typeof window.mermaid.init === 'function') {
      window.mermaid.init(undefined, document.querySelectorAll('pre.mermaid'));
    }
  });
</script>

<svelte:head>
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>stovoy.dev</title>

  <script src="https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.min.js"></script>
  <script>
    if (window.mermaid) {
      mermaid.initialize({ startOnLoad: true });
    }
  </script>

  <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/highlight.js@11.9.0/styles/tokyo-night-dark.min.css" />
  <script src="https://cdn.jsdelivr.net/npm/highlight.js@11.9.0/lib/common.min.js"></script>
  <script>
    if (window.hljs) {
      const run = () => hljs.highlightAll();
      if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', run);
      } else {
        run();
      }
    }
  </script>
</svelte:head>

<main>
  <slot />
</main>

<CodeViewer />

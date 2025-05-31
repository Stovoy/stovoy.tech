<script lang="ts">
  import { onMount } from 'svelte';

  export let filePath: string | undefined = undefined;

  let isOpen = false;
  let content: string = '';

  function toggle() {
    isOpen = !isOpen;
    if (isOpen) {
      loadContent();
    }
  }

  async function loadContent() {
    if (typeof window !== 'undefined' && !(window as any).hljs) {
      const script = document.createElement('script');
      script.src = '/highlight.min.js';
      script.onload = () => {
        if ((window as any).hljs) {
          (window as any).hljs.highlightAll();
        }
      };
      document.head.appendChild(script);
    }

    let path = filePath;
    if (!path && typeof window !== 'undefined') {
      if (window.location.hash.startsWith('#src=')) {
        path = decodeURIComponent(window.location.hash.slice(5));
      }
    }
    if (!path) {
      content = 'No file selected.';
      return;
    }

    try {
      const response = await fetch(`/source/${path}`);
      if (response.ok) {
        content = await response.text();
      } else {
        content = `Failed to load ${path}: ${response.statusText}`;
      }
    } catch (err) {
      content = `Error loading ${path}`;
    }

    if (typeof window !== 'undefined' && (window as any).hljs) {
      (window as any).hljs.highlightAll();
    }
  }

  onMount(() => {
    if (typeof window !== 'undefined' && window.location.hash.startsWith('#src=')) {
      isOpen = true;
      loadContent();
    }
  });
</script>

{#if isOpen}
  <div class="fixed inset-0 bg-black bg-opacity-60 z-40 flex">
    <div class="bg-white dark:bg-gray-900 m-auto max-w-screen-md max-h-screen overflow-auto p-4 rounded shadow-lg">
      <pre><code class="whitespace-pre-wrap text-xs">{content}</code></pre>
    </div>
  </div>
{/if}

<button on:click={toggle} class="fixed bottom-4 right-4 z-50 bg-blue-600 text-white rounded-full w-12 h-12 flex items-center justify-center shadow-lg">
  &lt;/&gt;
</button>

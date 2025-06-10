<script lang="ts">
  import { onMount, tick } from 'svelte';
  import { writable } from 'svelte/store';
  import { afterNavigate } from '$app/navigation';

  import { codeFiles, setCodeViewerOpen } from '$lib/stores/codeFiles';

  export let files: string[] | undefined = undefined;

  let fileList: string[] = [];

  const isOpenStore = writable(false);
  const selectedIndexStore = writable(0);
  const contentsStore = writable<Record<string, string>>({});

  function toggle() {
    isOpenStore.update((v) => {
      const newValue = !v;
      setCodeViewerOpen(newValue);
      return newValue;
    });
  }

  function close() {
    isOpenStore.set(false);
    setCodeViewerOpen(false);
  }

  function detectLanguageFromPath(path: string): string {
    const ext = path.split('.').pop()?.toLowerCase();
    const languageMap: Record<string, string> = {
      'js': 'javascript',
      'ts': 'typescript', 
      'jsx': 'javascript',
      'tsx': 'typescript',
      'svelte': 'javascript',
      'rs': 'rust',
      'py': 'python',
      'html': 'html',
      'css': 'css',
      'scss': 'css',
      'json': 'json',
      'md': 'markdown',
      'yml': 'yaml',
      'yaml': 'yaml',
      'toml': 'toml',
      'xml': 'xml',
      'sh': 'bash',
      'bash': 'bash'
    };
    return languageMap[ext || ''] || 'plaintext';
  }

  // Promise cache to avoid loading the highlighter multiple times
  let highlighterPromise: Promise<void> | null = null;

  function ensureHighlighter(): Promise<void> {
    if (typeof window === 'undefined') return Promise.resolve();
    if ((window as any).Prism || (window as any).hljs) return Promise.resolve();

    if (highlighterPromise) return highlighterPromise;

    highlighterPromise = new Promise((resolve, reject) => {
      const script = document.createElement('script');
      script.src = '/highlight.min.js';
      script.onload = () => resolve();
      script.onerror = () => reject();
      document.head.appendChild(script);
    });

    return highlighterPromise;
  }

  function applyHighlighting() {
    if (typeof window === 'undefined') return;
    if ((window as any).Prism) {
      (window as any).Prism.highlightAll();
    } else if ((window as any).hljs) {
      (window as any).hljs.highlightAll();
    }
  }

  async function loadContents(list: string[]) {
    if (list.length === 0) return;

    const entries: Record<string, string> = {};
    await Promise.all(
      list.map(async (path) => {
        try {
          const res = await fetch(`/source/${path}`);
          if (res.ok) {
            entries[path] = await res.text();
          } else {
            entries[path] = `Failed to load ${path}: ${res.statusText}`;
          }
        } catch (_) {
          entries[path] = `Error loading ${path}`;
        }
      })
    );
    contentsStore.set(entries);

    await tick();
    await ensureHighlighter();
    applyHighlighting();
  }

  function switchToTab(index: number) {
    if (index >= 0 && index < fileList.length) {
      selectedIndexStore.set(index);
      // Re-highlight the new content after DOM updates
      (async () => {
        await tick();
        await ensureHighlighter();
        applyHighlighting();
      })();
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape' && $isOpenStore) {
      close();
    }
  }

  let codeFilesUnsubscribe: (() => void) | null = null;

  export function isViewerOpen() {
    return $isOpenStore;
  }

  onMount(() => {
    let initialFiles: string[] | undefined = files;

    if (!initialFiles && typeof window !== 'undefined') {
      if (window.location.hash.startsWith('#src=')) {
        initialFiles = [decodeURIComponent(window.location.hash.slice(5))];
      }
    }

    if (initialFiles && initialFiles.length > 0) {
      fileList = [...initialFiles];
      loadContents(fileList);
    }

    codeFilesUnsubscribe = codeFiles.subscribe((list) => {
      if (list && list.length > 0) {
        fileList = [...list];
        selectedIndexStore.set(0);
        loadContents(fileList);
      }
    });

    // Listen for keyboard events
    window.addEventListener('keydown', handleKeydown);

    return () => {
      if (codeFilesUnsubscribe) {
        codeFilesUnsubscribe();
      }
      window.removeEventListener('keydown', handleKeydown);
    };
  });

  // Close viewer when navigating to a new page, but preserve files if viewer is open
  afterNavigate(() => {
    close();
  });

  $: currentFile = fileList[$selectedIndexStore];
  $: currentContent = $contentsStore[currentFile] || '';
  $: currentLanguage = currentFile ? detectLanguageFromPath(currentFile) : 'plaintext';

  // Re-highlight when content changes
  $: if (currentContent) {
    (async () => {
      await tick();
      await ensureHighlighter();
      applyHighlighting();
    })();
  }

  // Apply highlighting whenever the viewer is opened
  $: if ($isOpenStore) {
    (async () => {
      await tick();
      await ensureHighlighter();
      applyHighlighting();
    })();
  }
</script>

<!-- Show Code Button - Always in Top Right -->
<button
  on:click={toggle}
  class="code-viewer-button fixed top-4 right-4 z-50"
  style:display={fileList.length > 0 ? 'block' : 'none'}>
  {#if $isOpenStore}
    ✕ Close Code
  {:else}
    💻 View Code
  {/if}
</button>

{#if $isOpenStore}
  <!-- Full Screen Overlay -->
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div class="fixed inset-0 z-40 code-viewer-overlay" on:click={close}>
    <!-- Modal Content -->
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div class="absolute inset-8 code-viewer-modal rounded-xl overflow-hidden flex flex-col" on:click|stopPropagation>
      
      <!-- Header with tabs only -->
      <div class="p-4 border-b border-opacity-20 border-white">
        <!-- File Name or Tabs -->
        {#if fileList && fileList.length > 1}
          <div class="code-viewer-tabs flex overflow-x-auto">
            {#each fileList as path, idx (path)}
              <button
                class="code-viewer-tab"
                class:active={$selectedIndexStore === idx}
                on:click={() => switchToTab(idx)}
                title={path}
                type="button">
                {path.split('/').pop()}
              </button>
            {/each}
          </div>
        {:else if fileList && fileList.length === 1}
          <h3 class="text-lg font-medium text-blue-300">{fileList[0].split('/').pop()}</h3>
        {:else}
          <h3 class="text-lg font-medium text-gray-400">Code Viewer</h3>
        {/if}
      </div>

      <!-- Content area -->
      <div class="flex-1 overflow-hidden code-viewer-content">
        {#if fileList && fileList.length > 0 && currentContent}
          {#key currentFile}
            <pre class="code-viewer-pre h-full language-{currentLanguage}"><code class="code-viewer-code language-{currentLanguage}">{currentContent}</code></pre>
          {/key}
        {:else}
          <div class="flex items-center justify-center h-full text-center p-8">
            <div class="space-y-4">
              <div class="text-6xl opacity-20">📄</div>
              <div class="text-xl text-gray-400">No code selected</div>
              <div class="text-sm text-gray-500">Visit a page with code examples or append <code class="px-2 py-1 bg-black bg-opacity-30 rounded">#src=path/to/file.ext</code> to the URL</div>
            </div>
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}


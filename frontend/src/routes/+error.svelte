<script>
  export let error;
  export let status;
</script>

<svelte:head>
  <title>{status}</title>
</svelte:head>

<main class="flex flex-col items-center justify-center min-h-screen text-center p-8 space-y-4">
  <h1 class="text-6xl font-bold">{status}</h1>
  {#if status === 404}
    <p>The page you are looking for could not be found.</p>
  {:else}
    <p>{error?.message ?? 'Something went wrong.'}</p>
  {/if}
  <a href="/" class="text-blue-500 underline">Return home</a>
</main>

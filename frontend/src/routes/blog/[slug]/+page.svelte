<script lang="ts">
  export let data: { html: string };
</script>

<article class="prose max-w-none">
  {@html data.html}
</article>

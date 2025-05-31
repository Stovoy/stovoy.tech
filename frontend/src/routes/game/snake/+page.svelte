<script lang="ts">
  import SnakeGame from '$lib/components/SnakeGame.svelte';
</script>

<div class="back-button-container">
  <a href="/" class="back-button">
    <span class="back-arrow">←</span>
    <span>Back</span>
  </a>
</div>

<h1 class="text-2xl font-bold mb-4">Snake</h1>

<SnakeGame />

<script lang="ts">
  import { onMount } from 'svelte';

  export let text: string;
  export let charPerMs: number = 80;

  let displayed = '';
  let cursorVisible = true;

  onMount(() => {
    let index = 0;
    const interval = setInterval(() => {
      if (index < text.length) {
        displayed = text.slice(0, index + 1);
        index += 1;
      } else {
        clearInterval(interval);
      }
    }, charPerMs);

    const cursorInterval = setInterval(() => {
      cursorVisible = !cursorVisible;
    }, 400);

    return () => {
      clearInterval(interval);
      clearInterval(cursorInterval);
    };
  });
</script>

{displayed}<span class={cursorVisible ? '' : 'invisible'}>|</span>
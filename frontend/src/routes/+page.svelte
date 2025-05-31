<script lang="ts">
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';

  const commandSegments = [
    { text: 'stovoy', class: 'cmd-user' },
    { text: '@devbox', class: 'cmd-host' },
    { text: ' ~ ', class: 'cmd-path' },
    { text: '$ ', class: 'cmd-prompt' },
    { text: 'whoami', class: 'cmd-cmd' }
  ];

  const commandStr = commandSegments.map((s) => s.text).join('');

  let typed = '';
  let cursorVisible = true;
  let showRest = false;

  function segmentsForTyped(t: string) {
    const result: { text: string; class: string }[] = [];
    let offset = 0;
    for (const seg of commandSegments) {
      const end = Math.min(offset + seg.text.length, t.length);
      if (offset < t.length) {
        const part = t.slice(offset, end);
        if (part) result.push({ text: part, class: seg.class });
      }
      offset += seg.text.length;
    }
    return result;
  }

  let segments: { text: string; class: string }[] = [];

  onMount(() => {
    let index = 0;
    const typingInterval = setInterval(() => {
      if (index < commandStr.length) {
        typed = commandStr.slice(0, index + 1);
        segments = segmentsForTyped(typed);
        index += 1;
      } else {
        clearInterval(typingInterval);
        setTimeout(() => {
          showRest = true;
        }, 200);
      }
    }, 80);

    const cursorInterval = setInterval(() => {
      cursorVisible = !cursorVisible;
    }, 400);

    return () => {
      clearInterval(typingInterval);
      clearInterval(cursorInterval);
    };
  });
</script>

<div class="terminal">
  <div class="fake-command-line">
    {#each segments as seg}
      <span class={seg.class}>{seg.text}</span>
    {/each}
    {#if !showRest}
      {#if cursorVisible}<span class="cursor">|</span>{/if}
    {/if}
  </div>

  {#if showRest}
    <div transition:fade={{ duration: 500 }}>
      <p>Hi. I'm Steve, but you can call me Stovoy - 20+ years code wizard, creator of Evades.io, and Safety Engineer @ OpenAI</p>

      <div class="fake-command-line">
        <span class="cmd-user">stovoy</span><span class="cmd-host">@devbox</span><span class="cmd-path"> ~ </span><span class="cmd-prompt">$ </span><span class="cmd-cmd">cat interests.md</span>
      </div>
      <ul>
        <li>Rust | Godot | Optimizations</li>
        <li>Finnley (my dog) | Video Games | Building Cool Things | Sci-Fi</li>
        <li>Automate. Everything.</li>
      </ul>

      <div class="fake-command-line fake-command-line-spaced">
        <span class="cmd-user">stovoy</span><span class="cmd-host">@devbox</span><span class="cmd-path"> ~ </span><span class="cmd-prompt">$ </span><span class="cmd-cmd">ls projects</span>
      </div>
      <ul class="links">
        <li><a href="https://evades.io" target="_blank">Evades.io</a></li>
        <li><a href="/game/snake">Snake</a></li>
      </ul>

      <div class="fake-command-line fake-command-line-spaced">
        <span class="cmd-user">stovoy</span><span class="cmd-host">@devbox</span><span class="cmd-path"> ~ </span><span class="cmd-prompt">$ </span><span class="cmd-cmd">contact</span>
      </div>
      <ul class="links">
        <li><a href="https://github.com/stovoy" target="_blank">GitHub</a></li>
        <li><a href="https://twitch.tv/stovoy" target="_blank">Twitch</a></li>
      </ul>

      <p>This terminal styled with the <a href="https://github.com/catppuccin" target="_blank">Catppuccin theme</a></p>
    </div>
  {/if}
</div>

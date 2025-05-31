<script lang="ts">
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';
  import { registerCodeFiles } from '$lib/stores/codeFiles';

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

  let blogs: { title: string; date: string; slug: string }[] = [];

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
    registerCodeFiles(['frontend/src/routes/+page.svelte']);
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

    // Load blog metadata at build time – the glob is evaluated eagerly by Vite
    const mods = import.meta.glob('content/*.md', { eager: true, as: 'raw' }) as Record<string, string>;
    blogs = Object.entries(mods).map(([path, text]) => {
      const lines = text.split(/\r?\n/);
      let title = '';
      let date = '';
      for (const line of lines) {
        if (!title && line.startsWith('#')) title = line.replace(/^#+/, '').trim();
        if (!date && line.toLowerCase().startsWith('date:')) date = line.split(':', 2)[1]?.trim() ?? '';
        if (title && date) break;
      }
      const slug = path.substring(path.lastIndexOf('/') + 1).replace(/\.md$/, '');
      return { title, date, slug };
    }).sort((a, b) => b.date.localeCompare(a.date));

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
        <span class="cmd-user">stovoy</span><span class="cmd-host">@devbox</span><span class="cmd-path">&nbsp;~&nbsp;</span><span class="cmd-prompt">$&nbsp;</span><span class="cmd-cmd">cat interests.md</span>
      </div>
      <ul>
        <li>Rust | Godot | Optimizations</li>
        <li>Finnley (my dog) | Video Games | Building Cool Things | Sci-Fi</li>
        <li>Automate. Everything.</li>
      </ul>

      <div class="fake-command-line fake-command-line-spaced">
        <span class="cmd-user">stovoy</span><span class="cmd-host">@devbox</span><span class="cmd-path">&nbsp;~&nbsp;</span><span class="cmd-prompt">$&nbsp;</span><span class="cmd-cmd">ls projects</span>
      </div>
      <ul class="links">
        <li><a href="https://evades.io" target="_blank">Evades.io</a></li>
        <li><a href="/game/snake">Snake</a></li>
      </ul>

      {#if blogs.length}
        <div class="fake-command-line fake-command-line-spaced">
          <span class="cmd-user">stovoy</span><span class="cmd-host">@devbox</span><span class="cmd-path">&nbsp;~&nbsp;</span><span class="cmd-prompt">$&nbsp;</span><span class="cmd-cmd">ls blog</span>
        </div>
        <ul class="links">
          {#each blogs.slice(0, 5) as blog}
            <li>
              <a href={`/blog/${blog.slug}`}>{blog.title}</a>
              <span class="opacity-60 text-sm">&nbsp;{blog.date}</span>
            </li>
          {/each}
          {#if blogs.length > 5}
            <li><a href="/blog" class="text-blue-600 dark:text-blue-400 hover:underline">See all posts</a></li>
          {/if}
        </ul>
      {/if}

      <div class="fake-command-line fake-command-line-spaced">
        <span class="cmd-user">stovoy</span><span class="cmd-host">@devbox</span><span class="cmd-path">&nbsp;~&nbsp;</span><span class="cmd-prompt">$&nbsp;</span><span class="cmd-cmd">contact</span>
      </div>
      <ul class="links">
        <li><a href="https://github.com/stovoy" target="_blank">GitHub</a></li>
        <li><a href="https://twitch.tv/stovoy" target="_blank">Twitch</a></li>
      </ul>

      <p>This terminal styled with the <a href="https://github.com/catppuccin" target="_blank">Catppuccin theme</a></p>
    </div>
  {/if}
</div>

export function toHtml(markdown: string): string {
  const lines = markdown.replace(/\r\n?/g, '\n').split('\n');
  const html: string[] = [];
  let inList = false;
  let inCodeBlock = false;
  let codeLanguage = '';
  let codeLines: string[] = [];

  function closeList() {
    if (inList) {
      html.push('</ul>');
      inList = false;
    }
  }

  for (const raw of lines) {
    const line = raw.trim();

    if (inCodeBlock) {
      if (line.startsWith('```')) {
        const content = codeLines.join('\n');
        if (codeLanguage === 'dolphin' || codeLanguage === 'mermaid') {
          html.push(`<pre class="mermaid">${content}</pre>`);
        } else if (codeLanguage) {
          html.push(`<pre><code class="language-${escapeHtml(codeLanguage)}">${escapeHtml(content)}</code></pre>`);
        } else {
          html.push(`<pre><code>${escapeHtml(content)}</code></pre>`);
        }
        inCodeBlock = false;
        codeLanguage = '';
        codeLines = [];
      } else {
        codeLines.push(raw.replace(/^\s*/, ''));
      }
      continue;
    }

    if (line.startsWith('```')) {
      closeList();
      inCodeBlock = true;
      codeLanguage = line.slice(3).trim().toLowerCase();
      codeLines = [];
      continue;
    }

    if (!line) {
      closeList();
      continue;
    }

    if (line.startsWith('# ')) {
      closeList();
      html.push(`<h1>${renderInline(line.slice(2).trim())}</h1>`);
    } else if (line.startsWith('## ')) {
      closeList();
      html.push(`<h2>${renderInline(line.slice(3).trim())}</h2>`);
    } else if (line.startsWith('### ')) {
      closeList();
      html.push(`<h3>${renderInline(line.slice(4).trim())}</h3>`);
    } else if (line.startsWith('- ') || line.startsWith('* ') || line.startsWith('• ')) {
      if (!inList) {
        html.push('<ul>');
        inList = true;
      }
      html.push(`<li>${renderInline(line.slice(2).trim())}</li>`);
    } else if (line.toLowerCase().startsWith('date:')) {
      const value = escapeHtml(line.slice(5).trim());
      html.push(`<p><em>${value}</em></p>`);
    } else {
      closeList();
      html.push(`<p>${renderInline(line)}</p>`);
    }
  }

  closeList();

  return html.join('');
}

function processFormatting(str: string): string {
  const segments = str.split(/(<[^>]+>)/g);
  let output = '';
  for (const segment of segments) {
    if (segment.startsWith('<')) {
      output += segment;
      continue;
    }
    let s = escapeHtml(segment);
    s = s.replace(/\*\*\*(.+?)\*\*\*/g, '<strong><em>$1</em></strong>');
    s = s.replace(/___(.+?)___/g, '<strong><em>$1</em></strong>');
    s = s.replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>');
    s = s.replace(/__(.+?)__/g, '<strong>$1</strong>');
    s = s.replace(/\*(.+?)\*/g, '<em>$1</em>');
    s = s.replace(/_(.+?)_/g, '<em>$1</em>');
    output += s;
  }
  return output;
}

function renderInline(text: string): string {
  const imgRegex = /!\[([^\]]*)\]\(([^\)]+)\)/g;
  let str = text.replace(imgRegex, (_m, alt: string, url: string) => {
    const resolved = url.startsWith('http') || url.startsWith('/') ? url : `/img/${url}`;
    return `<img src="${escapeHtml(resolved)}" alt="${escapeHtml(alt)}">`;
  });

  const linkRegex = /\[([^\]]+)\]\(([^\)]+)\)/g;
  let result = '';
  let lastIndex = 0;

  for (const match of str.matchAll(linkRegex)) {
    const index = match.index ?? 0;
    const [full, label, url] = match;
    result += processFormatting(str.slice(lastIndex, index));
    result += `<a href="${escapeHtml(url)}">${processFormatting(label)}</a>`;
    lastIndex = index + full.length;
  }

  result += processFormatting(str.slice(lastIndex));
  return result;
}

function escapeHtml(str: string): string {
  return str
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#39;');
}

export interface BlogMeta {
  title: string;
  date: string;
  slug: string;
}

export function extractMeta(markdown: string, filePath: string): BlogMeta {
  const lines = markdown.replace(/\r\n?/g, '\n').split('\n');
  let title = '';
  let date = '';

  for (const line of lines) {
    if (!title && line.startsWith('#')) {
      title = line.replace(/^#+/, '').trim();
    }
    if (!date && line.toLowerCase().startsWith('date:')) {
      date = line.split(':', 2)[1]?.trim() ?? '';
    }
    if (title && date) break;
  }

  if (!title) {
    const pathParts = filePath.split('/');
    title = pathParts[pathParts.length - 1].replace(/\.md$/, '');
  }

  if (!date) {
    date = 'unknown';
  }

  const slug = filePath.substring(filePath.lastIndexOf('/') + 1).replace(/\.md$/, '');

  return { title, date, slug };
}
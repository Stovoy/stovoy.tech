export function toHtml(markdown: string): string {
  const lines = markdown.replace(/\r\n?/g, '\n').split('\n');
  const html: string[] = [];
  let inList = false;

  function closeList() {
    if (inList) {
      html.push('</ul>');
      inList = false;
    }
  }

  for (const raw of lines) {
    const line = raw.trim();

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
  let s = escapeHtml(str);
  s = s.replace(/\*\*\*(.+?)\*\*\*/g, '<strong><em>$1</em></strong>');
  s = s.replace(/___(.+?)___/g, '<strong><em>$1</em></strong>');
  s = s.replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>');
  s = s.replace(/__(.+?)__/g, '<strong>$1</strong>');
  s = s.replace(/\*(.+?)\*/g, '<em>$1</em>');
  s = s.replace(/_(.+?)_/g, '<em>$1</em>');
  return s;
}

function renderInline(text: string): string {
  const linkRegex = /\[([^\]]+)\]\(([^\)]+)\)/g;
  let result = '';
  let lastIndex = 0;

  for (const match of text.matchAll(linkRegex)) {
    const index = match.index ?? 0;
    const [full, label, url] = match;
    result += processFormatting(text.slice(lastIndex, index));
    result += `<a href="${escapeHtml(url)}">${processFormatting(label)}</a>`;
    lastIndex = index + full.length;
  }

  result += processFormatting(text.slice(lastIndex));
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
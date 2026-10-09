// Stories are stored as plain text with light Markdown: blank-line separated blocks,
// `## ` headings, `> ` quotes, `- ` / `1. ` lists, **bold**, *italic* and [label](https://url).
// Plain stories without any markup are valid and render exactly as before.

const esc = (s) =>
  s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');

const INLINE = /\*\*(.+?)\*\*|\*([^*\s][^*]*?)\*|\[([^\]]+)\]\((https?:\/\/[^\s)]+)\)/g;

/** Split inline markup into [{ text, bold, italic, href }] segments. */
export function parseInline(text, style = {}) {
  const out = [];
  let last = 0;
  for (const m of text.matchAll(INLINE)) {
    if (m.index > last) out.push({ text: text.slice(last, m.index), ...style });
    if (m[1] !== undefined) out.push(...parseInline(m[1], { ...style, bold: true }));
    else if (m[2] !== undefined) out.push(...parseInline(m[2], { ...style, italic: true }));
    else out.push(...parseInline(m[3], { ...style, href: m[4] }));
    last = m.index + m[0].length;
  }
  if (last < text.length) out.push({ text: text.slice(last), ...style });
  return out;
}

/** Split text into [leading space, core, trailing space] so translation can't eat the spaces. */
export function padSplit(text) {
  const m = /^(\s*)([\s\S]*?)(\s*)$/.exec(text);
  return [m[1], m[2], m[3]];
}

/** Parse a story into blocks: heading | quote | ul | ol | paragraph. */
export function parseBlocks(md) {
  return (md || '')
    .replace(/\r\n?/g, '\n')
    .split(/\n{2,}/)
    .map((b) => b.trim())
    .filter(Boolean)
    .map((b) => {
      const lines = b.split('\n');
      const heading = lines.length === 1 && /^(#{1,3})\s+(.+)$/.exec(b);
      if (heading) return { type: 'heading', level: Math.max(2, heading[1].length), text: heading[2] };
      if (lines.every((l) => /^>\s?/.test(l)))
        return { type: 'quote', text: lines.map((l) => l.replace(/^>\s?/, '')).join('\n') };
      if (lines.every((l) => /^[-*]\s+/.test(l)))
        return { type: 'ul', items: lines.map((l) => l.replace(/^[-*]\s+/, '')) };
      if (lines.every((l) => /^\d+[.)]\s+/.test(l)))
        return { type: 'ol', items: lines.map((l) => l.replace(/^\d+[.)]\s+/, '')) };
      return { type: 'p', text: b };
    });
}

/** Story text with all markup removed (cards, SEO descriptions). */
export function plainText(md) {
  return parseBlocks(md)
    .map((b) =>
      b.items
        ? b.items.map((i) => parseInline(i).map((s) => s.text).join('')).join('\n')
        : parseInline(b.text).map((s) => s.text).join('')
    )
    .join('\n\n');
}

const inlineHtml = (text) =>
  parseInline(text)
    .map((s) => {
      let h = esc(s.text).replace(/\n/g, '<br>');
      if (s.bold) h = `<strong>${h}</strong>`;
      if (s.italic) h = `<em>${h}</em>`;
      if (s.href) h = `<a href="${esc(s.href)}">${h}</a>`;
      return h;
    })
    .join('');

/** Markdown -> HTML for the editor (everything is escaped first, so it is safe to inject). */
export function toHtml(md) {
  return parseBlocks(md)
    .map((b) => {
      if (b.type === 'heading') return `<h${b.level}>${inlineHtml(b.text)}</h${b.level}>`;
      if (b.type === 'quote') return `<blockquote><p>${inlineHtml(b.text)}</p></blockquote>`;
      if (b.type === 'ul') return `<ul>${b.items.map((i) => `<li>${inlineHtml(i)}</li>`).join('')}</ul>`;
      if (b.type === 'ol') return `<ol>${b.items.map((i) => `<li>${inlineHtml(i)}</li>`).join('')}</ol>`;
      return `<p>${inlineHtml(b.text)}</p>`;
    })
    .join('');
}

function wrap(mark, inner) {
  const [lead, core, trail] = padSplit(inner);
  return core ? `${lead}${mark}${core}${mark}${trail}` : inner;
}

function inlineMd(node) {
  let out = '';
  for (const c of node.childNodes) {
    if (c.nodeType === 3) {
      out += c.nodeValue.replace(/ /g, ' ');
    } else if (c.nodeType === 1) {
      const tag = c.tagName;
      if (tag === 'BR') {
        out += '\n';
        continue;
      }
      const inner = inlineMd(c);
      if (tag === 'STRONG' || tag === 'B') out += wrap('**', inner);
      else if (tag === 'EM' || tag === 'I') out += wrap('*', inner);
      else if (tag === 'A' && /^https?:\/\//i.test(c.getAttribute('href') || '') && inner.trim())
        out += `[${inner}](${c.getAttribute('href')})`;
      else out += inner;
    }
  }
  return out;
}

const tidyInline = (s) => s.replace(/\n{2,}/g, '\n').trim();

/** Editor DOM -> Markdown. */
export function toMarkdown(root) {
  const blocks = [];
  const push = (s) => s && blocks.push(s);
  for (const node of root.childNodes) {
    if (node.nodeType === 3) {
      push(tidyInline(node.nodeValue));
    } else if (node.nodeType === 1) {
      const tag = node.tagName;
      if (/^H[1-6]$/.test(tag)) {
        const text = tidyInline(inlineMd(node)).replace(/\n/g, ' ');
        push(text && `${tag === 'H1' || tag === 'H2' ? '##' : '###'} ${text}`);
      } else if (tag === 'BLOCKQUOTE') {
        const text = tidyInline(inlineMd(node));
        push(text && text.split('\n').map((l) => `> ${l}`).join('\n'));
      } else if (tag === 'UL' || tag === 'OL') {
        const items = [...node.children]
          .map((li) => tidyInline(inlineMd(li)).replace(/\n/g, ' '))
          .filter(Boolean);
        push(items.map((t, i) => (tag === 'UL' ? `- ${t}` : `${i + 1}. ${t}`)).join('\n'));
      } else {
        push(tidyInline(inlineMd(node)));
      }
    }
  }
  return blocks.join('\n\n');
}

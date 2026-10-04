import { writable, get } from 'svelte/store';
import { browser } from '$app/environment';

export const lang = writable('en');
// translated strings, keyed by `${target}\u0000${source}\u0000${text}`
export const dict = writable({});

export const key = (target, source, text) => `${target}\u0000${source}\u0000${text}`;

/** Visitor's language: saved choice, else browser language ("bn-BD" -> "bn"). */
export function detectLang() {
  if (!browser) return 'en';
  try {
    const saved = localStorage.getItem('lang');
    if (saved) return saved;
  } catch {}
  const nav = (navigator.languages?.[0] || navigator.language || 'en').toLowerCase();
  if (nav.startsWith('zh-tw') || nav.startsWith('zh-hk')) return 'zt';
  return nav.split('-')[0];
}

export function setLang(code) {
  lang.set(code);
  try {
    localStorage.setItem('lang', code);
  } catch {}
  if (browser) document.documentElement.lang = code;
}

let queue = new Map(); // `${target}\u0000${source}` -> Set<text>
let timer = null;

/** Ask for a translation; batched into one request per (target, source). */
export function want(text, target, source = 'auto') {
  if (!browser || !text || !text.trim() || target === source) return;
  const k = key(target, source, text);
  if (get(dict)[k] !== undefined) return;
  const group = `${target}\u0000${source}`;
  if (!queue.has(group)) queue.set(group, new Set());
  queue.get(group).add(text);
  clearTimeout(timer);
  timer = setTimeout(flush, 40);
}

async function flush() {
  const batches = queue;
  queue = new Map();
  for (const [group, set] of batches) {
    const [target, source] = group.split('\u0000');
    const texts = [...set];
    try {
      const res = await fetch('/api/translate', {
        method: 'POST',
        headers: { 'content-type': 'application/json' },
        body: JSON.stringify({ texts, target, source })
      });
      if (!res.ok) throw new Error(String(res.status));
      const { translations } = await res.json();
      dict.update((d) => {
        texts.forEach((t, i) => (d[key(target, source, t)] = translations[i]));
        return { ...d };
      });
    } catch {
      // service down: keep original text
    }
  }
}

import { SITE_URL } from '$lib/seo.js';

const PAGE_SIZE = 100;
const MAX_URLS = 50_000;

function escapeXml(value) {
  return String(value)
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;')
    .replaceAll("'", '&apos;');
}

function urlEntry(path, lastModified) {
  const lastmod = lastModified ? new Date(lastModified).toISOString() : null;
  return `  <url><loc>${escapeXml(new URL(path, SITE_URL).href)}</loc>${lastmod ? `<lastmod>${lastmod}</lastmod>` : ''}</url>`;
}

export async function GET({ fetch }) {
  const urls = [urlEntry('/'), urlEntry('/people')];
  let offset = 0;
  let total = 0;

  do {
    const response = await fetch(`/api/people?limit=${PAGE_SIZE}&offset=${offset}`);
    if (!response.ok) {
      console.error(`Sitemap people request failed: ${response.status}`);
      return new Response('Unable to generate sitemap.', { status: 502 });
    }

    const data = await response.json();
    if (!Array.isArray(data.items) || !Number.isFinite(data.total)) {
      console.error('Sitemap people response had an invalid shape.');
      return new Response('Unable to generate sitemap.', { status: 502 });
    }

    total = Math.min(data.total, MAX_URLS - urls.length);
    for (const person of data.items) {
      if (urls.length >= MAX_URLS) break;
      urls.push(urlEntry(`/people/${encodeURIComponent(person.id)}`, person.created_at));
    }
    offset += data.items.length;
    if (data.items.length === 0) break;
  } while (offset < total && urls.length < MAX_URLS);

  const xml = `<?xml version="1.0" encoding="UTF-8"?>\n<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n${urls.join('\n')}\n</urlset>\n`;
  return new Response(xml, {
    headers: {
      'Content-Type': 'application/xml; charset=utf-8',
      'Cache-Control': 'public, max-age=3600'
    }
  });
}

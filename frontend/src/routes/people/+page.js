import { error } from '@sveltejs/kit';

const PAGE_SIZE = 20;

export async function load({ fetch, url }) {
  const q = url.searchParams.get('q') || '';
  const country = url.searchParams.get('country') || '';
  const page = Math.max(1, parseInt(url.searchParams.get('page') || '1', 10) || 1);
  const res = await fetch(
    `/api/people?limit=${PAGE_SIZE}&offset=${(page - 1) * PAGE_SIZE}&q=${encodeURIComponent(q)}&country=${encodeURIComponent(country)}`
  );
  if (!res.ok) error(res.status, 'Unable to load people right now. Please try again later.');
  const data = await res.json();
  return {
    q,
    country,
    page,
    people: data.items,
    total: data.total,
    pages: Math.max(1, Math.ceil(data.total / PAGE_SIZE))
  };
}

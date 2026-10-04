const PAGE_SIZE = 24;

export async function load({ fetch, url }) {
  const q = url.searchParams.get('q') || '';
  const page = Math.max(1, parseInt(url.searchParams.get('page') || '1', 10) || 1);
  const res = await fetch(
    `/api/people?limit=${PAGE_SIZE}&offset=${(page - 1) * PAGE_SIZE}&q=${encodeURIComponent(q)}`
  );
  const data = res.ok ? await res.json() : { items: [], total: 0 };
  return {
    q,
    page,
    people: data.items,
    total: data.total,
    pages: Math.max(1, Math.ceil(data.total / PAGE_SIZE))
  };
}

const PAGE_SIZE = 20;

export async function load({ fetch, url }) {
  const page = Math.max(1, parseInt(url.searchParams.get('page') || '1', 10) || 1);
  const res = await fetch(`/api/people?limit=${PAGE_SIZE}&offset=${(page - 1) * PAGE_SIZE}`);
  const data = res.ok ? await res.json() : { items: [], total: 0 };
  return {
    page,
    all: data.items,
    total: data.total,
    pages: Math.max(1, Math.ceil(data.total / PAGE_SIZE))
  };
}

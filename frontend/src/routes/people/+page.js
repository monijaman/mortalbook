export async function load({ fetch, url }) {
  const q = url.searchParams.get('q') || '';
  const res = await fetch(`/api/people?limit=48&q=${encodeURIComponent(q)}`);
  return { q, people: res.ok ? await res.json() : [] };
}

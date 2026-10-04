import { error } from '@sveltejs/kit';

export async function load({ fetch, params }) {
  const res = await fetch(`/api/people/${params.id}`);
  if (res.status === 404) error(404, 'Not found');
  if (!res.ok) error(502, 'Could not load this page');
  return { person: await res.json() };
}

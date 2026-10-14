<script>
  import { onMount } from 'svelte';
  import { adminApi, logout } from '$lib/admin.js';

  const PAGE = 25;
  let items = $state([]);
  let total = $state(0);
  let offset = $state(0);
  let query = $state('');
  let error = $state('');
  let notice = $state('');
  let loading = $state(false);

  onMount(() => load());

  async function load() {
    loading = true;
    error = '';
    try {
      const params = new URLSearchParams({ limit: PAGE, offset });
      if (query.trim()) params.set('q', query.trim());
      const data = await adminApi(`/api/admin/people?${params}`);
      items = data.items;
      total = data.total;
    } catch (err) {
      error = err.message;
    } finally {
      loading = false;
    }
  }

  function search(event) {
    event.preventDefault();
    offset = 0;
    load();
  }

  function page(delta) {
    offset = Math.max(0, offset + delta * PAGE);
    load();
  }

  async function remove(person) {
    if (!confirm(`Permanently delete ${person.name}? This cannot be undone.`)) return;
    error = '';
    notice = '';
    try {
      await adminApi(`/api/admin/people/${person.id}`, { method: 'DELETE' });
      notice = `${person.name} was deleted.`;
      await load();
    } catch (err) {
      error = err.message;
    }
  }
</script>

<svelte:head>
  <title>Manage people — Mortalbook</title>
  <meta name="robots" content="noindex,nofollow" />
</svelte:head>

<h1>Manage people</h1>

<div class="toolbar">
  <a class="button" href="/admin/new">+ Add a person</a>
  <a href="/admin/review">Review recent-death candidates</a>
  <button type="button" class="secondary" onclick={logout}>Sign out</button>
</div>

<form class="search" onsubmit={search}>
  <input type="search" placeholder="Search by name" bind:value={query} />
  <button type="submit">Search</button>
</form>

{#if error}<p class="error" role="alert">{error}</p>{/if}
{#if notice}<p class="notice" role="status">{notice}</p>{/if}

<p class="muted">{total} {total === 1 ? 'person' : 'people'}{loading ? ' · loading…' : ''}</p>

<table>
  <thead>
    <tr><th>Name</th><th>Born</th><th>Died</th><th>Occupation</th><th></th></tr>
  </thead>
  <tbody>
    {#each items as person (person.id)}
      <tr>
        <td><a href={`/people/${person.id}`}>{person.name}</a></td>
        <td>{person.birth_date ?? '—'}</td>
        <td>{person.death_date}</td>
        <td>{person.occupation ?? '—'}</td>
        <td class="actions">
          <a href={`/admin/people/${person.id}`}>Edit</a>
          <button type="button" class="danger" onclick={() => remove(person)}>Delete</button>
        </td>
      </tr>
    {/each}
  </tbody>
</table>

<div class="toolbar">
  <button type="button" onclick={() => page(-1)} disabled={offset === 0 || loading}>← Previous</button>
  <button type="button" onclick={() => page(1)} disabled={offset + PAGE >= total || loading}>Next →</button>
</div>

<style>
  .toolbar { display: flex; flex-wrap: wrap; gap: 0.75rem; align-items: center; margin: 1rem 0; }
  .search { display: flex; gap: 0.5rem; max-width: 500px; }
  .search input { flex: 1; }
  table { width: 100%; border-collapse: collapse; }
  th, td { text-align: left; padding: 0.5rem 0.6rem; border-bottom: 1px solid var(--line); }
  th { color: var(--muted); font-weight: 500; }
  .actions { display: flex; gap: 0.75rem; align-items: center; white-space: nowrap; }
  .secondary { background: transparent; color: var(--text); }
  .danger { background: var(--danger); }
  .button { background: var(--accent); color: #fff; padding: 0.45rem 0.9rem; border-radius: 6px; text-decoration: none; }
  .error { color: var(--danger); }
  .notice { color: var(--accent); }
  @media (max-width: 640px) { th:nth-child(4), td:nth-child(4), th:nth-child(2), td:nth-child(2) { display: none; } }
</style>

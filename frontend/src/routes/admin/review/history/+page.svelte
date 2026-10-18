<script>
  import { onMount } from 'svelte';
  import { adminApi, logout } from '$lib/admin.js';

  const PAGE = 25;
  let items = $state([]);
  let total = $state(0);
  let offset = $state(0);
  let query = $state('');
  let status = $state('');
  let error = $state('');
  let loading = $state(false);
  let runningCron = $state(false);

  onMount(() => load());

  async function runCron() {
    runningCron = true;
    error = '';
    try {
      await adminApi('/api/admin/recent-deaths/run', { method: 'POST' });
      await load();
    } catch (err) {
      error = err.message;
    } finally {
      runningCron = false;
    }
  }

  async function load() {
    loading = true;
    error = '';
    try {
      const params = new URLSearchParams({ limit: PAGE, offset });
      if (query.trim()) params.set('q', query.trim());
      if (status) params.set('status', status);
      const data = await adminApi(`/api/admin/recent-deaths/history?${params}`);
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
</script>

<svelte:head>
  <title>Cron import history — Mortalbook</title>
  <meta name="robots" content="noindex,nofollow" />
</svelte:head>

<h1>Cron import history</h1>
<p class="muted">
  Recent-death candidates submitted by the scheduled import, including their current review status.
</p>

<div class="toolbar">
  <a href="/admin">← All people</a>
  <a href="/admin/review">Review pending candidates</a>
  <button type="button" class="button" onclick={runCron} disabled={runningCron}>{runningCron ? 'Starting…' : 'Run cron once'}</button>
  <button type="button" class="secondary" onclick={logout}>Sign out</button>
</div>

<form class="filters" onsubmit={search}>
  <input type="search" placeholder="Search by name" bind:value={query} />
  <select bind:value={status} aria-label="Filter by review status">
    <option value="">All statuses</option>
    <option value="pending">Pending</option>
    <option value="approved">Approved</option>
    <option value="rejected">Rejected</option>
  </select>
  <button type="submit">Filter</button>
</form>

{#if error}<p class="error" role="alert">{error}</p>{/if}
<p class="muted">{total} {total === 1 ? 'candidate' : 'candidates'}{loading ? ' · loading…' : ''}</p>

<table>
  <thead>
    <tr><th>Imported</th><th>Name</th><th>Date of passing</th><th>Status</th><th>Sources</th><th>Actions</th></tr>
  </thead>
  <tbody>
    {#each items as item (item.id)}
      <tr>
        <td><time datetime={item.created_at}>{new Date(item.created_at).toLocaleString()}</time></td>
        <td>{item.name}</td>
        <td>{item.death_date}</td>
        <td>
          {item.status}
          {#if item.reviewed_at}
            <span class="muted"><br />{new Date(item.reviewed_at).toLocaleString()}</span>
          {/if}
        </td>
        <td class="sources">
          <a href={item.wikidata_url} target="_blank" rel="noopener noreferrer">Wikidata</a>
          ·
          <a href={item.wikipedia_url} target="_blank" rel="noopener noreferrer">Wikipedia</a>
        </td>
        <td>
          {#if item.status === 'pending'}
            <a class="edit" href={`/admin/review?edit=${item.id}`}>Edit</a>
          {:else}
            <span class="muted">—</span>
          {/if}
        </td>
      </tr>
    {/each}
  </tbody>
</table>

{#if !loading && items.length === 0}
  <p class="muted">No imported candidates match these filters.</p>
{/if}

<div class="toolbar">
  <button type="button" onclick={() => page(-1)} disabled={offset === 0 || loading}>← Previous</button>
  <button type="button" onclick={() => page(1)} disabled={offset + PAGE >= total || loading}>Next →</button>
</div>

<style>
  .toolbar, .filters { display: flex; flex-wrap: wrap; gap: 0.75rem; align-items: center; margin: 1rem 0; }
  .filters input { flex: 1; min-width: 200px; }
  select { background: var(--surface); color: var(--text); border: 1px solid var(--line); border-radius: 6px; padding: 0.6rem 0.75rem; font: inherit; }
  table { width: 100%; border-collapse: collapse; }
  th, td { text-align: left; padding: 0.5rem 0.6rem; border-bottom: 1px solid var(--line); }
  .sources { white-space: nowrap; }
  .secondary { background: transparent; color: var(--text); }
  .button { background: var(--accent); color: #fff; padding: 0.45rem 0.9rem; border-radius: 6px; text-decoration: none; }
  .error { color: var(--danger); }
  @media (max-width: 640px) {
    table { font-size: 0.9rem; }
    th, td { padding: 0.45rem 0.3rem; }
    .sources { white-space: normal; }
  }
</style>

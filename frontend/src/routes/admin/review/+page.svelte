<script>
  import { onMount } from 'svelte';
  import { adminApi, logout } from '$lib/admin.js';
  import StoryEditor from '$lib/StoryEditor.svelte';

  let people = $state([]);
  let error = $state('');
  let notice = $state('');
  let busyId = $state('');
  let loading = $state(false);
  let runningCron = $state(false);
  let page = $state(0);
  const pageSize = 10;
  let editingId = $state('');
  let selectedIds = $state(new Set());
  let pagePeople = $derived(people.slice(page * pageSize, (page + 1) * pageSize));
  let pageCount = $derived(Math.max(1, Math.ceil(people.length / pageSize)));
  let allSelected = $derived(pagePeople.length > 0 && pagePeople.every((person) => selectedIds.has(person.id)));

  onMount(() => {
    loadPeople().catch((err) => { error = err.message; });
  });

  async function loadPeople() {
    loading = true;
    error = '';
    try {
      people = await adminApi('/api/admin/pending-people');
      page = Math.min(page, Math.max(0, Math.ceil(people.length / pageSize) - 1));
    } finally {
      loading = false;
    }
  }

  async function runCron() {
    runningCron = true;
    error = '';
    notice = '';
    try {
      await adminApi('/api/admin/recent-deaths/run', { method: 'POST' });
      notice = 'Cron job started. Refresh the queue in a moment.';
    } catch (err) {
      error = err.message;
    } finally {
      runningCron = false;
    }
  }

  function editable(person) {
    return {
      name: person.name,
      birth_date: person.birth_date || null,
      death_date: person.death_date,
      bio: person.bio,
      occupation: person.occupation || null,
      birth_place: person.birth_place || null,
      death_place: person.death_place || null
    };
  }

  async function save(person) {
    busyId = person.id;
    error = '';
    notice = '';
    try {
      await adminApi(`/api/admin/pending-people/${person.id}`, {
        method: 'PATCH',
        body: JSON.stringify(editable(person))
      });
      notice = `${person.name} saved.`;
    } catch (err) {
      error = err.message;
    } finally {
      busyId = '';
    }
  }

  async function decide(person, decision) {
    if (decision === 'reject' && !confirm(`Reject ${person.name}?`)) return;
    busyId = person.id;
    error = '';
    notice = '';
    try {
      if (decision === 'approve') {
        await adminApi(`/api/admin/pending-people/${person.id}`, {
          method: 'PATCH',
          body: JSON.stringify(editable(person))
        });
      }
      await adminApi(`/api/admin/pending-people/${person.id}/${decision}`, { method: 'POST' });
      people = people.filter((item) => item.id !== person.id);
      editingId = '';
      page = Math.min(page, Math.max(0, Math.ceil(people.length / pageSize) - 1));
      notice = decision === 'approve'
        ? `${person.name} was approved and added to the public catalog.`
        : `${person.name} was rejected.`;
    } catch (err) {
      error = err.message;
    } finally {
      busyId = '';
    }
  }

  function toggleSelected(id) {
    const next = new Set(selectedIds);
    next.has(id) ? next.delete(id) : next.add(id);
    selectedIds = next;
  }

  function toggleAll() {
    const next = new Set(selectedIds);
    if (allSelected) pagePeople.forEach((person) => next.delete(person.id));
    else pagePeople.forEach((person) => next.add(person.id));
    selectedIds = next;
  }

  async function makeActive() {
    if (!selectedIds.size || !confirm(`Make ${selectedIds.size} selected candidate(s) active?`)) return;
    error = '';
    try {
      const result = await adminApi('/api/admin/pending-people/bulk-approve', {
        method: 'POST', body: JSON.stringify({ ids: [...selectedIds] })
      });
      people = people.filter((person) => !selectedIds.has(person.id));
      selectedIds = new Set();
      notice = `${result.published} candidate(s) are now active and public.`;
    } catch (err) { error = err.message; }
  }

</script>

<svelte:head>
  <title>Review recent deaths — Mortalbook</title>
  <meta name="robots" content="noindex,nofollow" />
</svelte:head>

<h1>Review recent-death candidates</h1>
<p class="muted">
  Candidates are private until approved. Verify their death and biographical details against
  reliable reporting, edit the story, then approve or reject each record.
</p>

<div class="toolbar">
  <a href="/admin">← All people</a>
  <a href="/admin/review/history">Cron import history</a>
  <button type="button" class="button" onclick={runCron} disabled={runningCron}>{runningCron ? 'Starting…' : 'Run cron once'}</button>
  <button
    type="button"
    onclick={() => loadPeople().catch((err) => { error = err.message; })}
    disabled={loading}
  >Refresh queue</button>
  <button type="button" class="secondary" onclick={logout}>Sign out</button>
</div>

{#if error}<p class="error" role="alert">{error}</p>{/if}
{#if notice}<p class="notice" role="status">{notice}</p>{/if}
{#if loading}<p class="muted">Loading review queue…</p>{/if}
{#if !loading && people.length === 0}
  <p class="muted">There are no recent-death candidates waiting for review.</p>
{/if}

<section class="list" aria-label="Pending recent-death candidates">
  {#if selectedIds.size}<div class="bulk-actions"><span>{selectedIds.size} selected</span><button type="button" class="approve" onclick={makeActive}>Make active</button></div>{/if}
  <table>
    <thead><tr><th><input type="checkbox" aria-label="Select all on this page" checked={allSelected} onchange={toggleAll} /></th><th>Name</th><th>Date of passing</th><th>Occupation</th><th></th></tr></thead>
    <tbody>
    {#each pagePeople as person (person.id)}
      <tr>
        <td><input type="checkbox" aria-label={`Select ${person.name}`} checked={selectedIds.has(person.id)} onchange={() => toggleSelected(person.id)} /></td>
        <td>{person.name}</td>
        <td>{person.death_date}</td>
        <td>{person.occupation || '—'}</td>
        <td><button type="button" onclick={() => editingId = person.id}>Edit</button></td>
      </tr>
    {/each}
    </tbody>
  </table>
</section>

{#if editingId}
  {@const person = people.find((item) => item.id === editingId)}
  {#if person}
    <div class="modal-backdrop" role="presentation" onclick={(event) => { if (event.target === event.currentTarget) editingId = ''; }}>
      <div class="candidate modal" role="dialog" aria-modal="true" aria-labelledby="edit-title">
      <div class="candidate-header"><h2 id="edit-title">Edit {person.name}</h2><button type="button" class="secondary" onclick={() => editingId = ''}>Close</button></div>
      <p class="sources">
        <a href={person.wikidata_url} target="_blank" rel="noopener noreferrer">Wikidata record</a>
        ·
        <a href={person.wikipedia_url} target="_blank" rel="noopener noreferrer">English Wikipedia</a>
      </p>
      <form class="editor" onsubmit={(event) => { event.preventDefault(); save(person); }}>
        <label>
          <span>Name</span>
          <input bind:value={person.name} maxlength="200" required />
        </label>
        <div class="row">
          <label>
            <span>Birth date (if verified)</span>
            <input type="date" bind:value={person.birth_date} />
          </label>
          <label>
            <span>Date of passing</span>
            <input type="date" bind:value={person.death_date} required />
          </label>
        </div>
        <label>
          <span>Occupation</span>
          <input bind:value={person.occupation} maxlength="500" />
        </label>
        <div class="row">
          <label>
            <span>Birth place</span>
            <input bind:value={person.birth_place} maxlength="300" />
          </label>
          <label>
            <span>Death place</span>
            <input bind:value={person.death_place} maxlength="300" />
          </label>
        </div>
        <StoryEditor bind:value={person.bio} context={person} label="Verified story (required before approval)" />
        <div class="actions">
          <button type="submit" disabled={busyId === person.id}>
            {busyId === person.id ? 'Saving…' : 'Save edits'}
          </button>
          <button
            type="button"
            class="approve"
            onclick={() => decide(person, 'approve')}
            disabled={busyId === person.id}
          >Approve and publish</button>
          <button
            type="button"
            class="reject"
            onclick={() => decide(person, 'reject')}
            disabled={busyId === person.id}
          >Reject</button>
        </div>
      </form>
      </div>
    </div>
  {/if}
{/if}

{#if people.length > 0}
  <div class="pagination" aria-label="Review queue pagination">
    <button type="button" onclick={() => page -= 1} disabled={page === 0}>← Previous</button>
    <span class="muted">Page {page + 1} of {pageCount}</span>
    <button type="button" onclick={() => page += 1} disabled={page + 1 >= pageCount}>Next →</button>
  </div>
{/if}

<style>
  .editor { display: grid; gap: 1rem; }
  .list { overflow-x: auto; margin-top: 1.5rem; }
  .bulk-actions { display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.75rem; }
  table { width: 100%; border-collapse: collapse; }
  th, td { text-align: left; padding: 0.7rem 0.6rem; border-bottom: 1px solid var(--line); }
  th { color: var(--muted); font-weight: 500; }
  .candidate { border: 1px solid var(--line); border-radius: 10px; padding: 1.25rem; }
  .modal-backdrop { position: fixed; inset: 0; z-index: 10; display: grid; place-items: center; padding: 1rem; background: rgb(0 0 0 / 0.65); }
  .modal { width: min(760px, 100%); max-height: calc(100vh - 2rem); overflow-y: auto; background: var(--surface); box-shadow: 0 1rem 3rem rgb(0 0 0 / 0.35); }
  .candidate h2 { margin: 0; }
  .candidate-header { display: flex; justify-content: space-between; gap: 1rem; align-items: center; }
  .sources { margin: 0.5rem 0 1rem; }
  .sources a { color: var(--accent); }
  label span { display: block; margin-bottom: 0.3rem; color: var(--muted); }
  .row { display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; }
  .toolbar, .actions, .pagination { display: flex; flex-wrap: wrap; gap: 0.75rem; align-items: center; }
  .toolbar { margin: 1.2rem 0; }
  .actions { padding-top: 0.2rem; }
  .secondary { background: transparent; color: var(--text); }
  .button { background: var(--accent); color: #fff; padding: 0.45rem 0.9rem; border-radius: 6px; text-decoration: none; }
  .approve { background: var(--accent); }
  .reject { background: var(--danger); }
  .error { color: var(--danger); }
  .notice { color: var(--accent); }
  .pagination { justify-content: center; margin: 1.25rem 0; }
  @media (max-width: 640px) { .row { grid-template-columns: 1fr; } }
</style>

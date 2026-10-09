<script>
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import { adminApi } from '$lib/admin.js';
  import StoryEditor from '$lib/StoryEditor.svelte';

  const PRECISIONS = ['day', 'month', 'year', 'decade', 'century'];
  const id = $page.params.id;
  let person = $state(null);
  let error = $state('');
  let notice = $state('');
  let busy = $state(false);
  let saved = $state('');
  const dirty = $derived(person !== null && JSON.stringify(person) !== saved);

  function onKeydown(event) {
    if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === 's') {
      event.preventDefault();
      if (person && !busy) save(event);
    }
  }
  function beforeUnload(event) {
    if (dirty) event.preventDefault();
  }

  onMount(async () => {
    try {
      // Auth check first so signed-out visitors land on the login page.
      await adminApi('/api/admin/me');
      const res = await fetch(`/api/people/${id}`);
      if (!res.ok) throw new Error('Person not found.');
      person = await res.json();
      saved = JSON.stringify(person);
    } catch (err) {
      error = err.message;
    }
  });

  async function save(event) {
    event.preventDefault();
    busy = true;
    error = '';
    notice = '';
    try {
      const blank = (v) => (v && String(v).trim() ? v : null);
      await adminApi(`/api/admin/people/${id}`, {
        method: 'PATCH',
        body: JSON.stringify({
          name: person.name,
          birth_date: blank(person.birth_date),
          death_date: person.death_date,
          bio: person.bio,
          occupation: blank(person.occupation),
          birth_place: blank(person.birth_place),
          death_place: blank(person.death_place),
          birth_precision: person.birth_precision,
          death_precision: person.death_precision
        })
      });
      saved = JSON.stringify(person);
      notice = 'Saved.';
    } catch (err) {
      error = err.message;
    } finally {
      busy = false;
    }
  }

  async function remove() {
    if (!confirm(`Permanently delete ${person.name}? This cannot be undone.`)) return;
    try {
      await adminApi(`/api/admin/people/${id}`, { method: 'DELETE' });
      await goto('/admin');
    } catch (err) {
      error = err.message;
    }
  }
</script>

<svelte:head>
  <title>Edit person — Mortalbook</title>
  <meta name="robots" content="noindex,nofollow" />
</svelte:head>

<svelte:window onkeydown={onKeydown} onbeforeunload={beforeUnload} />

<p><a href="/admin">← All people</a></p>
<h1>Edit person</h1>

{#if error}<p class="error" role="alert">{error}</p>{/if}
{#if notice}<p class="notice" role="status">{notice}</p>{/if}

{#if person}
  <form onsubmit={save}>
    <label><span>Full name *</span><input bind:value={person.name} maxlength="200" required /></label>
    <div class="row">
      <label><span>Date of birth</span><input type="date" bind:value={person.birth_date} /></label>
      <label>
        <span>Birth date precision</span>
        <select bind:value={person.birth_precision}>
          {#each PRECISIONS as p}<option value={p}>{p}</option>{/each}
        </select>
      </label>
    </div>
    <div class="row">
      <label><span>Date of passing *</span><input type="date" bind:value={person.death_date} required /></label>
      <label>
        <span>Death date precision</span>
        <select bind:value={person.death_precision}>
          {#each PRECISIONS as p}<option value={p}>{p}</option>{/each}
        </select>
      </label>
    </div>
    <label><span>Occupation</span><input bind:value={person.occupation} maxlength="500" /></label>
    <div class="row">
      <label><span>Birth place</span><input bind:value={person.birth_place} maxlength="300" /></label>
      <label><span>Death place</span><input bind:value={person.death_place} maxlength="300" /></label>
    </div>
    <StoryEditor bind:value={person.bio} />
    <div class="actions">
      <button type="submit" disabled={busy || !dirty}>{busy ? 'Saving…' : 'Save changes (Ctrl+S)'}</button>
      {#if dirty}<span class="muted">Unsaved changes</span>{/if}
      <a href={`/people/${id}`}>View public page</a>
      <button type="button" class="danger" onclick={remove}>Delete</button>
    </div>
  </form>
{/if}

<style>
  form { display: grid; gap: 1.1rem; max-width: 1100px; margin-top: 1rem; }
  .row, label { max-width: 680px; }
  label span { display: block; margin-bottom: 0.3rem; color: var(--muted); font-size: 0.95rem; }
  .row { display: grid; gap: 1rem; grid-template-columns: 1fr 1fr; }
  .actions { display: flex; flex-wrap: wrap; gap: 1rem; align-items: center; }
  .danger { background: var(--danger); margin-left: auto; }
  .error { color: var(--danger); }
  .notice { color: var(--accent); }
  @media (max-width: 640px) { .row { grid-template-columns: 1fr; } }
</style>

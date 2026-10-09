<script>
  import { onMount } from 'svelte';
  import { adminApi, logout } from '$lib/admin.js';
  import StoryEditor from '$lib/StoryEditor.svelte';

  let people = $state([]);
  let error = $state('');
  let notice = $state('');
  let busyId = $state('');
  let loading = $state(false);

  onMount(() => {
    loadPeople().catch((err) => { error = err.message; });
  });

  async function loadPeople() {
    loading = true;
    error = '';
    try {
      people = await adminApi('/api/admin/pending-people');
    } finally {
      loading = false;
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
      notice = decision === 'approve'
        ? `${person.name} was approved and added to the public catalog.`
        : `${person.name} was rejected.`;
    } catch (err) {
      error = err.message;
    } finally {
      busyId = '';
    }
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

<section class="queue" aria-label="Pending recent-death candidates">
  {#each people as person (person.id)}
    <article class="candidate">
      <h2>{person.name}</h2>
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
        <StoryEditor bind:value={person.bio} label="Verified story (required before approval)" />
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
    </article>
  {/each}
</section>

<style>
  .editor { display: grid; gap: 1rem; }
  .queue { display: grid; gap: 1.5rem; margin-top: 1.5rem; }
  .candidate { border: 1px solid var(--line); border-radius: 10px; padding: 1.25rem; }
  .candidate h2 { margin: 0; }
  .sources { margin: 0.5rem 0 1rem; }
  .sources a { color: var(--accent); }
  label span { display: block; margin-bottom: 0.3rem; color: var(--muted); }
  .row { display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; }
  .toolbar, .actions { display: flex; flex-wrap: wrap; gap: 0.75rem; align-items: center; }
  .toolbar { margin: 1.2rem 0; }
  .actions { padding-top: 0.2rem; }
  .secondary { background: transparent; color: var(--text); }
  .approve { background: var(--accent); }
  .reject { background: var(--danger); }
  .error { color: var(--danger); }
  .notice { color: var(--accent); }
  @media (max-width: 640px) { .row { grid-template-columns: 1fr; } }
</style>

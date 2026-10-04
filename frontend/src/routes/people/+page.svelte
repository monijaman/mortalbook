<script>
  import T from '$lib/T.svelte';
  import PersonCard from '$lib/PersonCard.svelte';
  import Pagination from '$lib/Pagination.svelte';

  let { data } = $props();

  const href = (n) => `/people?${new URLSearchParams({ ...(data.q ? { q: data.q } : {}), page: String(n) })}`;
</script>

<svelte:head><title>Remembered — Mortalbook</title></svelte:head>

<h1><T text="Remembered" /></h1>
<form method="GET" class="search">
  <input name="q" value={data.q} placeholder="Search by name" />
  <button type="submit"><T text="Search" /></button>
</form>

{#if data.people.length === 0}
  <p class="muted"><T text="No one found." /></p>
{:else}
  <p class="muted count">{data.total} · <T text="Page" /> {data.page} / {data.pages}</p>
  <div class="grid">
    {#each data.people as person (person.id)}
      <PersonCard {person} />
    {/each}
  </div>

  <Pagination page={data.page} pages={data.pages} {href} />
{/if}

<style>
  .search { display: flex; gap: 0.6rem; margin: 1rem 0 1rem; max-width: 480px; }
  .count { margin: 0 0 1rem; font-size: 0.95rem; }
</style>

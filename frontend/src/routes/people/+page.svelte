<script>
  import T from '$lib/T.svelte';
  import PersonCard from '$lib/PersonCard.svelte';
  import Pagination from '$lib/Pagination.svelte';
  import SearchBox from '$lib/SearchBox.svelte';

  let { data } = $props();

  const href = (n) => `/people?${new URLSearchParams({ ...(data.q ? { q: data.q } : {}), page: String(n) })}`;
</script>

<svelte:head><title>Remembered — Mortalbook</title></svelte:head>

<h1><T text="Remembered" /></h1>
<div class="find"><SearchBox value={data.q} /></div>

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
  .find { margin: 1rem 0 1rem; }
  .count { margin: 0 0 1rem; font-size: 0.95rem; }
</style>

<script>
  import T from '$lib/T.svelte';
  import PersonCard from '$lib/PersonCard.svelte';

  let { data } = $props();
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
  <div class="grid">
    {#each data.people as person (person.id)}
      <PersonCard {person} />
    {/each}
  </div>
{/if}

<style>
  .search { display: flex; gap: 0.6rem; margin: 1rem 0 2rem; max-width: 480px; }
</style>

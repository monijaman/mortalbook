<script>
  import T from '$lib/T.svelte';
  import PersonCard from '$lib/PersonCard.svelte';

  let { data } = $props();

  const href = (n) => `/people?${new URLSearchParams({ ...(data.q ? { q: data.q } : {}), page: String(n) })}`;

  // 1 … 4 5 [6] 7 8 … 20
  const numbers = $derived.by(() => {
    const out = [];
    const near = new Set([1, data.pages, data.page - 2, data.page - 1, data.page, data.page + 1, data.page + 2]);
    let last = 0;
    for (let n = 1; n <= data.pages; n++) {
      if (!near.has(n)) continue;
      if (n - last > 1) out.push('…');
      out.push(n);
      last = n;
    }
    return out;
  });
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

  {#if data.pages > 1}
    <nav class="pager" aria-label="Pagination">
      {#if data.page > 1}<a class="btn" href={href(data.page - 1)}>← <T text="Previous" /></a>{/if}
      {#each numbers as n}
        {#if n === '…'}
          <span class="gap">…</span>
        {:else if n === data.page}
          <span class="current" aria-current="page">{n}</span>
        {:else}
          <a href={href(n)}>{n}</a>
        {/if}
      {/each}
      {#if data.page < data.pages}<a class="btn" href={href(data.page + 1)}><T text="Next" /> →</a>{/if}
    </nav>
  {/if}
{/if}

<style>
  .search { display: flex; gap: 0.6rem; margin: 1rem 0 1rem; max-width: 480px; }
  .count { margin: 0 0 1rem; font-size: 0.95rem; }
  .pager { display: flex; flex-wrap: wrap; gap: 0.4rem; justify-content: center; align-items: center; margin: 2.5rem 0 0; }
  .pager a, .pager .current, .gap {
    min-width: 2.4rem; text-align: center; padding: 0.4rem 0.7rem; border-radius: 6px; text-decoration: none;
  }
  .pager a:not(.btn) { border: 1px solid var(--line); color: var(--text); }
  .pager a:not(.btn):hover { border-color: var(--accent); color: var(--accent); }
  .current { background: var(--accent); color: #17140c; }
  .gap { color: var(--muted); }
  .pager .btn { padding: 0.4rem 1rem; }
</style>

<script>
  import T from '$lib/T.svelte';

  // href: (pageNumber) => url
  let { page, pages, href } = $props();

  // 1 … 4 5 [6] 7 8 … 20
  const numbers = $derived.by(() => {
    const out = [];
    const near = new Set([1, pages, page - 2, page - 1, page, page + 1, page + 2]);
    let last = 0;
    for (let n = 1; n <= pages; n++) {
      if (!near.has(n)) continue;
      if (n - last > 1) out.push('…');
      out.push(n);
      last = n;
    }
    return out;
  });
</script>

{#if pages > 1}
  <nav class="pager" aria-label="Pagination">
    {#if page > 1}<a class="btn" href={href(page - 1)}>← <T text="Previous" /></a>{/if}
    {#each numbers as n}
      {#if n === '…'}
        <span class="gap">…</span>
      {:else if n === page}
        <span class="current" aria-current="page">{n}</span>
      {:else}
        <a href={href(n)}>{n}</a>
      {/if}
    {/each}
    {#if page < pages}<a class="btn" href={href(page + 1)}><T text="Next" /> →</a>{/if}
  </nav>
{/if}

<style>
  .pager { display: flex; flex-wrap: wrap; gap: 0.4rem; justify-content: center; align-items: center; margin: 2.5rem 0 0; }
  .pager a, .pager .current, .gap {
    min-width: 2.4rem; text-align: center; padding: 0.4rem 0.7rem; border-radius: 6px; text-decoration: none;
  }
  .pager a:not(.btn) { border: 1px solid var(--line); color: var(--text); }
  .pager a:not(.btn):hover { border-color: var(--accent); color: var(--accent); }
  .current { background: var(--accent); color: var(--accent-contrast); }
  .gap { color: var(--muted); }
  .pager .btn { padding: 0.4rem 1rem; }
</style>

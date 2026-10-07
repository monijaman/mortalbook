<script>
  import { lang, dict, key, want } from '$lib/i18n.js';

  let { value = '', country: initialCountry = '', large = false } = $props();
  let query = $state(value);
  let country = $state(initialCountry);
  let results = $state([]);
  let open = $state(false);
  let loading = $state(false);
  let timer;
  let request;

  // placeholders are attributes, so translate them by hand
  const hint = 'Search by name or profession…';
  $effect(() => {
    want(hint, $lang, 'en');
  });
  const placeholder = $derived($dict[key($lang, 'en', hint)] ?? hint);

  function search() {
    clearTimeout(timer);
    const q = query.trim();
    if (!q) {
      results = [];
      open = false;
      loading = false;
      return;
    }

    timer = setTimeout(async () => {
      request?.abort();
      request = new AbortController();
      loading = true;
      open = true;
      try {
        const res = await fetch(`/api/people?limit=8&offset=0&q=${encodeURIComponent(q)}&country=${encodeURIComponent(country)}`, {
          signal: request.signal
        });
        if (res.ok) results = (await res.json()).items ?? [];
      } catch (error) {
        if (error.name !== 'AbortError') results = [];
      } finally {
        loading = false;
      }
    }, 300);
  }

  function cleanup() {
    clearTimeout(timer);
    request?.abort();
  }
</script>

<svelte:window on:beforeunload={cleanup} />

<form class="search" class:large method="GET" action="/people" role="search" on:submit={() => cleanup()}>
  <select name="country" bind:value={country} aria-label="Filter by country" onchange={() => query.trim() && search()}>
    <option value="">All countries</option>
    <option>Bangladesh</option>
    <option>India</option>
    <option>Pakistan</option>
    <option>United Kingdom</option>
    <option>United States</option>
  </select>
  <div class="input-wrap">
    <input type="search" name="q" bind:value={query} {placeholder} aria-label={placeholder} autocomplete="off" on:input={search} on:focus={() => query.trim() && (open = true)} />
    {#if open}
      <div class="results" role="listbox" aria-label="Search results">
        {#if loading}
          <p class="status">Searching…</p>
        {:else if results.length}
          {#each results as person (person.id)}
            <a href={`/people/${person.id}`} role="option" on:click={() => (open = false)}>
              <strong>{person.name}</strong>
              {#if person.occupation}<span>{person.occupation}</span>{/if}
            </a>
          {/each}
          <a class="all" href={`/people?q=${encodeURIComponent(query.trim())}&country=${encodeURIComponent(country)}`}>See all results →</a>
        {:else}
          <p class="status">No results found.</p>
        {/if}
      </div>
    {/if}
  </div>
</form>

<style>
  .search { display: flex; gap: .6rem; max-width: 760px; }
  .search.large { max-width: 760px; margin: 0 auto; }
  select { flex: 0 0 150px; border-radius: 999px; padding: 0 .9rem; }
  .input-wrap { position: relative; }
  input { width: 100%; box-sizing: border-box; }
  .results {
    position: absolute; z-index: 5; top: calc(100% + 0.6rem); left: 0; right: 0; overflow: hidden;
    text-align: left; border: 1px solid var(--line); border-radius: 1rem; background: var(--surface, #17191d);
    box-shadow: 0 12px 30px rgba(0,0,0,.28);
  }
  .results a { display: block; padding: .75rem 1rem; color: inherit; text-decoration: none; }
  .results a:hover, .results a:focus { background: rgba(185, 167, 121, .12); }
  .results strong, .results span { display: block; }
  .results span, .status { color: var(--muted); font-size: .9rem; }
  .status { margin: 0; padding: .9rem 1rem; }
  .results .all { border-top: 1px solid var(--line); color: var(--accent); }
  .large input {
    font-size: 1.35rem; padding: 1rem 1.4rem; border-radius: 999px; border-color: var(--accent);
    background: rgba(23, 25, 29, 0.9); box-shadow: 0 0 0 4px rgba(185, 167, 121, 0.08);
  }
  @media (max-width: 520px) {
    select { flex-basis: 120px; }
    .large input { font-size: 1.1rem; padding: 0.8rem 1.1rem; }
  }
</style>

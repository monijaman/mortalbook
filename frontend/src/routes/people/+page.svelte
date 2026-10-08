<script>
  import { untrack } from 'svelte';
  import T from '$lib/T.svelte';
  import PersonCard from '$lib/PersonCard.svelte';
  import Pagination from '$lib/Pagination.svelte';
  import SearchBox from '$lib/SearchBox.svelte';
  import { selectedCountry } from '$lib/country.js';
  import { canonicalUrl, DEFAULT_DESCRIPTION } from '$lib/seo.js';

  let { data } = $props();

  let people = $state(untrack(() => data.people));
  let total = $state(untrack(() => data.total));
  let pages = $state(untrack(() => data.pages));
  let loading = $state(false);
  let failed = $state(false);
  const href = (n) => `/people?${new URLSearchParams({ ...(data.q ? { q: data.q } : {}), ...($selectedCountry ? { country: $selectedCountry } : {}), page: String(n) })}`;

  $effect(() => {
    const country = $selectedCountry;
    const params = new URLSearchParams({
      limit: '20',
      offset: String((data.page - 1) * 20),
      q: data.q
    });
    if (country) params.set('country', country);
    const controller = new AbortController();
    loading = true;
    failed = false;

    fetch(`/api/people?${params}`, { signal: controller.signal })
      .then((res) => {
        if (!res.ok) throw new Error(`People request failed: ${res.status}`);
        return res.json();
      })
      .then((result) => {
        people = result.items;
        total = result.total;
        pages = Math.max(1, Math.ceil(result.total / 20));
        failed = false;
      })
      .catch((error) => {
        if (error.name !== 'AbortError') {
          console.error(error);
          failed = true;
        }
      })
      .finally(() => {
        if (!controller.signal.aborted) loading = false;
      });

    return () => controller.abort();
  });
</script>

<svelte:head>
  <title>Remembered People — Mortalbook</title>
  <link rel="canonical" href={canonicalUrl('/people')} />
  <meta name="description" content={DEFAULT_DESCRIPTION} />
  <meta property="og:title" content="Remembered People — Mortalbook" />
  <meta property="og:description" content={DEFAULT_DESCRIPTION} />
  <meta property="og:url" content={canonicalUrl('/people')} />
  <meta name="twitter:title" content="Remembered People — Mortalbook" />
  {#if data.q || data.country || data.page > 1 || data.total === 0}
    <meta name="robots" content="noindex,follow" />
  {/if}
</svelte:head>

<h1><T text="Remembered" /></h1>
<div class="find"><SearchBox value={data.q} /></div>

{#if loading}
  <p class="muted"><T text="Loading…" /></p>
{:else if failed}
  <p class="muted"><T text="Something went wrong. Please try again later." /></p>
{:else if people.length === 0}
  <p class="muted"><T text="No one found." /></p>
{:else}
  <p class="muted count">{total} · <T text="Page" /> {data.page} / {pages}</p>
  <div class="grid">
    {#each people as person (person.id)}
      <PersonCard {person} />
    {/each}
  </div>

  <Pagination page={data.page} {pages} {href} />
{/if}

<style>
  .find { margin: 1rem 0 1rem; }
  .count { margin: 0 0 1rem; font-size: 0.95rem; }
</style>

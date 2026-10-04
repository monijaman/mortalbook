<script>
  import { onMount } from 'svelte';
  import T from '$lib/T.svelte';
  import PersonCard from '$lib/PersonCard.svelte';
  import Slideshow from '$lib/Slideshow.svelte';
  import Pagination from '$lib/Pagination.svelte';
  import { lang } from '$lib/i18n.js';

  let { data } = $props();

  let people = $state([]);
  let loading = $state(true);
  let failed = $state(false);
  let today = $state(new Date());

  onMount(async () => {
    // use the visitor's local date, not the server's
    today = new Date();
    try {
      const res = await fetch(`/api/people/today?month=${today.getMonth() + 1}&day=${today.getDate()}`);
      if (!res.ok) throw new Error();
      people = await res.json();
    } catch {
      failed = true;
    }
    loading = false;
  });

  const dateLabel = $derived(today.toLocaleDateString($lang, { month: 'long', day: 'numeric' }));
  const href = (n) => `/?page=${n}#all`;
</script>

<svelte:head><title>Mortalbook — remembered today</title></svelte:head>

<section class="hero">
  <p class="eyebrow"><T text="Remembered on this day" /></p>
  <h1>{dateLabel}</h1>
</section>

{#if loading}
  <p class="muted center"><T text="Loading…" /></p>
{:else if failed}
  <p class="muted center"><T text="Something went wrong. Please try again later." /></p>
{:else if people.length === 0}
  <p class="muted center empty">
    <T text="No one in our book passed away on this day." />
    <a href="/admin"><T text="Add someone you remember." /></a>
  </p>
{:else}
  <Slideshow {people} />

  <section class="block">
    <h2><T text="Also remembered today" /></h2>
    <div class="grid">
      {#each people as person (person.id)}
        <PersonCard {person} showAgo />
      {/each}
    </div>
  </section>
{/if}

<section class="block" id="all">
  <h2><T text="Everyone remembered" /></h2>
  {#if data.all.length === 0}
    <p class="muted"><T text="No one found." /></p>
  {:else}
    <p class="muted count">{data.total} · <T text="Page" /> {data.page} / {data.pages}</p>
    <div class="grid">
      {#each data.all as person (person.id)}
        <PersonCard {person} />
      {/each}
    </div>
    <Pagination page={data.page} pages={data.pages} {href} />
  {/if}
</section>

<style>
  .hero { text-align: center; padding: 1.5rem 0 2.5rem; }
  .eyebrow { color: var(--accent); text-transform: uppercase; letter-spacing: 0.2em; font-size: 0.85rem; margin: 0; }
  h1 { font-size: clamp(2.2rem, 6vw, 3.6rem); margin: 0.3rem 0 0; }
  .center { text-align: center; }
  .block { margin: 0 0 3.5rem; scroll-margin-top: 5rem; }
  .block h2 { margin: 0 0 1.2rem; padding-bottom: 0.5rem; border-bottom: 1px solid var(--line); }
  .count { margin: 0 0 1rem; font-size: 0.95rem; }
</style>

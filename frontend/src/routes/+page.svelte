<script>
  import { onMount } from 'svelte';
  import T from '$lib/T.svelte';
  import PersonCard from '$lib/PersonCard.svelte';
  import Slideshow from '$lib/Slideshow.svelte';
  import SearchBox from '$lib/SearchBox.svelte';
  import { lang } from '$lib/i18n.js';

  let people = $state([]);
  let upcoming = $state([]);
  let recent = $state([]);
  let loading = $state(true);
  let failed = $state(false);
  let today = $state(new Date());

  onMount(async () => {
    // use the visitor's local date, not the server's
    today = new Date();
    const md = `year=${today.getFullYear()}&month=${today.getMonth() + 1}&day=${today.getDate()}`;
    try {
      const [t, w] = await Promise.all([fetch(`/api/people/today?${md}`), fetch(`/api/people/week?${md}`)]);
      if (!t.ok) throw new Error();
      people = await t.json();
      if (w.ok) ({ upcoming, recent } = await w.json());
    } catch {
      failed = true;
    }
    loading = false;
  });

  const dateLabel = $derived(today.toLocaleDateString($lang, { month: 'long', day: 'numeric' }));
</script>

<svelte:head><title>Mortalbook — remembered today</title></svelte:head>

<section class="hero">
  <p class="eyebrow"><T text="Remembered on this day" /></p>
  <h1>{dateLabel}</h1>
  <div class="find"><SearchBox large /></div>
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

{#if upcoming.length}
  <section class="block">
    <h2><T text="Coming up this week" /></h2>
    <div class="grid">
      {#each upcoming as person (person.id)}
        <PersonCard {person} showAgo days={person.days} />
      {/each}
    </div>
  </section>
{/if}

{#if recent.length}
  <section class="block">
    <h2><T text="Last week" /></h2>
    <div class="grid">
      {#each recent as person (person.id)}
        <PersonCard {person} showAgo days={person.days} />
      {/each}
    </div>
  </section>
{/if}

<p class="center more"><a class="btn" href="/people"><T text="Browse everyone remembered" /> →</a></p>

<style>
  .hero { text-align: center; padding: 1.5rem 0 2.5rem; }
  .eyebrow { color: var(--accent); text-transform: uppercase; letter-spacing: 0.2em; font-size: 0.85rem; margin: 0; }
  h1 { font-size: clamp(2.2rem, 6vw, 3.6rem); margin: 0.3rem 0 0; }
  .find { margin-top: 2rem; }
  .center { text-align: center; }
  .block { margin: 0 0 3.5rem; }
  .block h2 { margin: 0 0 1.2rem; padding-bottom: 0.5rem; border-bottom: 1px solid var(--line); }
  .more { margin: 1rem 0 0; }
</style>
